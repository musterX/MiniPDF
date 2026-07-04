<template>
  <div class="flex flex-col h-screen bg-gray-50 select-none">
    <header class="h-12 flex items-center px-3 bg-white border-b border-gray-200 shrink-0" data-tauri-drag-region>
      <span class="text-sm font-semibold text-gray-700 w-20">MiniPDF</span>
      <template v-if="docPath">
        <span class="text-xs text-gray-500 truncate max-w-48">{{ docPath.split('/').pop() }}</span>
        <div class="ml-auto flex items-center gap-1">
          <div class="flex items-center gap-1 border-r border-gray-200 pr-2">
            <button @click="openFile" class="px-2 py-1 text-xs bg-gray-100 hover:bg-gray-200 rounded" title="打开">📂</button>
            <button @click="saveFile" class="px-2 py-1 text-xs bg-gray-100 hover:bg-gray-200 rounded" title="保存">💾</button>
            <button @click="saveAsFile" class="px-2 py-1 text-xs bg-gray-100 hover:bg-gray-200 rounded" title="另存为">📥</button>
          </div>
          <div class="flex items-center gap-1 border-r border-gray-200 pr-2">
            <button @click="showDeleteModal = true" class="px-2 py-1 text-xs bg-gray-100 hover:bg-gray-200 rounded" title="删除页面">🗑️</button>
            <button @click="showExtractModal = true" class="px-2 py-1 text-xs bg-gray-100 hover:bg-gray-200 rounded" title="提取页面">📄</button>
            <button @click="rotatePage" class="px-2 py-1 text-xs bg-gray-100 hover:bg-gray-200 rounded" title="旋转当前页">🔄</button>
            <button @click="showMergeModal = true" class="px-2 py-1 text-xs bg-gray-100 hover:bg-gray-200 rounded" title="合并 PDF">📑</button>
          </div>
          <div class="flex items-center gap-1 border-r border-gray-200 pr-2">
            <button @click="compressPdf" :disabled="loading" class="px-2 py-1 text-xs bg-green-100 hover:bg-green-200 rounded disabled:opacity-50" title="压缩">📦</button>
          </div>
          <div class="flex items-center gap-1">
            <button @click="textMode = !textMode" :class="['px-2 py-1 text-xs rounded', textMode ? 'bg-yellow-400 text-black' : 'bg-gray-100 hover:bg-gray-200']" title="文本模式">T</button>
            <button @click="prevPage" :disabled="currentPage <= 0" class="px-2 py-1 text-xs bg-gray-100 hover:bg-gray-200 rounded disabled:opacity-50">◀</button>
            <span class="text-xs text-gray-600 w-16 text-center">{{ currentPage + 1 }} / {{ pageCount }}</span>
            <button @click="nextPage" :disabled="currentPage >= pageCount - 1" class="px-2 py-1 text-xs bg-gray-100 hover:bg-gray-200 rounded disabled:opacity-50">▶</button>
            <button @click="zoomOut" class="px-2 py-1 text-xs bg-gray-100 hover:bg-gray-200 rounded">−</button>
            <span class="text-xs text-gray-600 w-12 text-center">{{ Math.round(scale * 100) }}%</span>
            <button @click="zoomIn" class="px-2 py-1 text-xs bg-gray-100 hover:bg-gray-200 rounded">+</button>
            <button @click="fitWidth" class="px-2 py-1 text-xs bg-gray-100 hover:bg-gray-200 rounded">适配</button>
          </div>
        </div>
      </template>
      <template v-else>
        <button @click="openFile" class="ml-4 px-3 py-1 text-xs bg-blue-500 text-white rounded hover:bg-blue-600">打开 PDF</button>
      </template>
    </header>

    <main class="flex-1 overflow-auto flex items-start justify-center bg-gray-100 p-4" ref="mainRef">
      <div class="relative inline-block" v-if="pageImageUrl">
        <img
          :src="pageImageUrl"
          :key="pageImageUrl"
          class="max-w-full shadow-lg bg-white block"
          :style="{ width: renderWidth + 'px' }"
          :class="{ 'cursor-crosshair': textMode }"
          @load="onImageLoad"
          @error="onImageError"
          @click="onPageClick"
        />
        <div
          v-for="ann in annotations"
          :key="ann.id"
          class="absolute cursor-pointer border border-transparent hover:border-blue-400"
          :style="annotStyle(ann)"
          @click.stop="onAnnotClick(ann)"
        >
          <span
            class="whitespace-pre-wrap break-words block"
            :style="{ fontSize: (ann.font_size * scale) + 'px', color: rgbStr(ann.color), lineHeight: 1.2 }"
          >{{ ann.text }}</span>
        </div>
      </div>
      <div v-else class="flex flex-col items-center justify-center h-full text-gray-400">
        <svg class="w-16 h-16 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
        </svg>
        <p class="text-sm">点击「打开 PDF」开始浏览</p>
      </div>
    </main>

    <div v-if="toast" class="fixed bottom-4 left-1/2 -translate-x-1/2 px-4 py-2 rounded shadow-lg text-sm z-50" :class="toast.type === 'error' ? 'bg-red-500 text-white' : toast.type === 'success' ? 'bg-green-500 text-white' : 'bg-gray-800 text-white'">
      {{ toast.message }}
    </div>

    <div v-if="loading" class="fixed inset-0 bg-black/30 flex items-center justify-center z-50">
      <div class="bg-white px-6 py-4 rounded shadow-lg text-sm">{{ loadingText }}</div>
    </div>

    <div v-if="editor" class="fixed bg-white shadow-lg rounded border border-gray-300 p-3 z-50" :style="{ left: editor.screenX + 'px', top: editor.screenY + 'px' }" @click.stop>
      <textarea ref="editorTextarea" v-model="editor.text" class="block border border-gray-300 rounded p-1 text-sm w-48 h-16 resize-none" placeholder="输入文本..."></textarea>
      <div class="flex items-center gap-2 mt-2">
        <input type="number" v-model.number="editor.fontSize" min="6" max="72" class="w-14 border border-gray-300 rounded px-1 text-xs" />
        <input type="color" v-model="editor.colorHex" class="w-6 h-6 p-0 border-0 cursor-pointer" />
        <button @click="saveEditor" class="px-2 py-1 text-xs bg-blue-500 text-white rounded hover:bg-blue-600">确定</button>
        <button v-if="editor.id !== null" @click="deleteAnnot" class="px-2 py-1 text-xs bg-red-500 text-white rounded hover:bg-red-600">删除</button>
        <button @click="editor = null" class="px-2 py-1 text-xs bg-gray-300 rounded hover:bg-gray-400">取消</button>
      </div>
    </div>

    <div v-if="showDeleteModal" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="showDeleteModal = false">
      <div class="bg-white rounded shadow-xl p-4 w-80">
        <h3 class="text-sm font-semibold mb-3">删除页面</h3>
        <p class="text-xs text-gray-500 mb-2">输入页码，用逗号分隔（如：1,3,5-7）</p>
        <input v-model="deletePagesInput" class="w-full border border-gray-300 rounded px-2 py-1 text-sm mb-3" placeholder="1,3,5-7" />
        <div class="flex justify-end gap-2">
          <button @click="showDeleteModal = false" class="px-3 py-1 text-xs bg-gray-200 rounded hover:bg-gray-300">取消</button>
          <button @click="doDeletePages" class="px-3 py-1 text-xs bg-red-500 text-white rounded hover:bg-red-600">删除</button>
        </div>
      </div>
    </div>

    <div v-if="showExtractModal" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="showExtractModal = false">
      <div class="bg-white rounded shadow-xl p-4 w-80">
        <h3 class="text-sm font-semibold mb-3">提取页面</h3>
        <p class="text-xs text-gray-500 mb-2">输入页码，用逗号分隔（如：1,3,5-7）</p>
        <input v-model="extractPagesInput" class="w-full border border-gray-300 rounded px-2 py-1 text-sm mb-3" placeholder="1,3,5-7" />
        <div class="flex justify-end gap-2">
          <button @click="showExtractModal = false" class="px-3 py-1 text-xs bg-gray-200 rounded hover:bg-gray-300">取消</button>
          <button @click="doExtractPages" class="px-3 py-1 text-xs bg-blue-500 text-white rounded hover:bg-blue-600">提取</button>
        </div>
      </div>
    </div>

    <div v-if="showMergeModal" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="showMergeModal = false">
      <div class="bg-white rounded shadow-xl p-4 w-96">
        <h3 class="text-sm font-semibold mb-3">合并 PDF</h3>
        <div class="mb-3 max-h-40 overflow-auto">
          <div v-for="(f, i) in mergeFiles" :key="i" class="flex items-center gap-2 text-xs py-1 border-b border-gray-100">
            <span class="flex-1 truncate">{{ f.split('/').pop() }}</span>
            <button @click="mergeFiles.splice(i, 1)" class="text-red-500 hover:text-red-700">✕</button>
          </div>
          <p v-if="mergeFiles.length === 0" class="text-xs text-gray-400 py-2">暂无文件</p>
        </div>
        <button @click="addMergeFile" class="w-full px-3 py-2 text-xs bg-gray-100 rounded hover:bg-gray-200 mb-3">+ 添加 PDF 文件</button>
        <div class="flex justify-end gap-2">
          <button @click="showMergeModal = false" class="px-3 py-1 text-xs bg-gray-200 rounded hover:bg-gray-300">取消</button>
          <button @click="doMerge" :disabled="mergeFiles.length < 2" class="px-3 py-1 text-xs bg-blue-500 text-white rounded hover:bg-blue-600 disabled:opacity-50">合并</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, nextTick } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open, save } from '@tauri-apps/plugin-dialog'

interface Annotation {
  id: number
  page: number
  x: number
  y: number
  width: number
  height: number
  text: string
  font_size: number
  color: [number, number, number]
}

interface EditorState {
  id: number | null
  text: string
  fontSize: number
  colorHex: string
  pdfX: number
  pdfY: number
  screenX: number
  screenY: number
}

const docPath = ref('')
const pageCount = ref(0)
const currentPage = ref(0)
const scale = ref(1.0)
const pageWidth = ref(612)
const pageHeight = ref(792)
const mainRef = ref<HTMLElement | null>(null)
const textMode = ref(false)
const annotations = ref<Annotation[]>([])
const editor = ref<EditorState | null>(null)
const editorTextarea = ref<HTMLTextAreaElement | null>(null)

const loading = ref(false)
const loadingText = ref('加载中...')
const toast = ref<{ message: string; type: 'success' | 'error' | 'info' } | null>(null)

const showDeleteModal = ref(false)
const showExtractModal = ref(false)
const showMergeModal = ref(false)
const deletePagesInput = ref('')
const extractPagesInput = ref('')
const mergeFiles = ref<string[]>([])

const renderWidth = computed(() => Math.round(pageWidth.value * scale.value))

const pageImageUrl = computed(() => {
  if (!docPath.value) return ''
  const encodedDoc = encodeURIComponent(docPath.value)
  return `pdfpage://localhost/page/${currentPage.value}?scale=${scale.value}&doc=${encodedDoc}`
})

function showToast(message: string, type: 'success' | 'error' | 'info' = 'info') {
  toast.value = { message, type }
  setTimeout(() => { toast.value = null }, 2500)
}

function parsePageRange(input: string): number[] {
  const pages: number[] = []
  for (const part of input.split(',')) {
    const trimmed = part.trim()
    if (!trimmed) continue
    const match = trimmed.match(/^(\d+)(?:-(\d+))?$/)
    if (match) {
      const start = parseInt(match[1]) - 1
      const end = match[2] ? parseInt(match[2]) - 1 : start
      for (let i = start; i <= end; i++) {
        if (i >= 0) pages.push(i)
      }
    }
  }
  return [...new Set(pages)].sort((a, b) => a - b)
}

function rgbStr(c: [number, number, number]) {
  return `rgb(${Math.round(c[0] * 255)},${Math.round(c[1] * 255)},${Math.round(c[2] * 255)})`
}

function hexToRgb(hex: string): [number, number, number] {
  const r = parseInt(hex.slice(1, 3), 16) / 255
  const g = parseInt(hex.slice(3, 5), 16) / 255
  const b = parseInt(hex.slice(5, 7), 16) / 255
  return [r, g, b]
}

function rgbToHex(c: [number, number, number]) {
  const to2 = (v: number) => Math.round(v * 255).toString(16).padStart(2, '0')
  return '#' + to2(c[0]) + to2(c[1]) + to2(c[2])
}

function annotStyle(ann: Annotation) {
  return {
    left: (ann.x * scale.value) + 'px',
    top: (ann.y * scale.value) + 'px',
    width: (ann.width * scale.value) + 'px',
    minHeight: (ann.height * scale.value) + 'px',
  }
}

async function loadAnnotations() {
  if (!docPath.value) return
  try {
    const raw = await invoke<string>('text_list', { input: docPath.value, page: currentPage.value })
    annotations.value = JSON.parse(raw)
  } catch {
    annotations.value = []
  }
}

function onPageClick(e: MouseEvent) {
  if (!textMode.value || editor.value) return
  const img = e.currentTarget as HTMLImageElement
  const rect = img.getBoundingClientRect()
  const clickX = e.clientX - rect.left
  const clickY = e.clientY - rect.top
  const pdfX = clickX / scale.value
  const pdfY = clickY / scale.value
  editor.value = {
    id: null,
    text: '',
    fontSize: 14,
    colorHex: '#000000',
    pdfX,
    pdfY,
    screenX: e.clientX,
    screenY: e.clientY + 10,
  }
  nextTick(() => editorTextarea.value?.focus())
}

function onAnnotClick(ann: Annotation) {
  if (textMode.value && editor.value) return
  editor.value = {
    id: ann.id,
    text: ann.text,
    fontSize: ann.font_size,
    colorHex: rgbToHex(ann.color),
    pdfX: ann.x,
    pdfY: ann.y,
    screenX: ann.x * scale.value + 10,
    screenY: ann.y * scale.value + ann.height * scale.value + 10,
  }
  nextTick(() => editorTextarea.value?.focus())
}

async function saveEditor() {
  if (!editor.value || !docPath.value) return
  const ed = editor.value
  const color = hexToRgb(ed.colorHex)
  try {
    if (ed.id === null) {
      const params = {
        input: docPath.value,
        page: currentPage.value,
        x: ed.pdfX,
        y: ed.pdfY,
        width: 200,
        height: ed.fontSize * 2,
        text: ed.text,
        font_size: ed.fontSize,
        color,
      }
      await invoke('text_add', { params: JSON.stringify(params) })
    } else {
      await invoke('text_edit', {
        input: docPath.value,
        page: currentPage.value,
        id: ed.id,
        text: ed.text,
        font_size: ed.fontSize,
        color,
      })
    }
    editor.value = null
    textMode.value = false
    await loadAnnotations()
    showToast('文本已保存', 'success')
  } catch (e) {
    showToast('保存失败: ' + e, 'error')
  }
}

async function deleteAnnot() {
  if (!editor.value || editor.value.id === null) return
  try {
    await invoke('text_delete', {
      input: docPath.value,
      page: currentPage.value,
      id: editor.value.id,
    })
    editor.value = null
    await loadAnnotations()
    showToast('文本已删除', 'success')
  } catch (e) {
    showToast('删除失败: ' + e, 'error')
  }
}

async function openFile() {
  const selected = await open({
    filters: [{ name: 'PDF', extensions: ['pdf'] }],
    multiple: false,
  })
  if (selected) {
    loading.value = true
    loadingText.value = '正在打开...'
    try {
      docPath.value = selected as string
      const result = await invoke<{ page_count: number; page_width: number; page_height: number }>('pdf_open', {
        input: docPath.value,
      })
      pageCount.value = result.page_count
      pageWidth.value = result.page_width
      pageHeight.value = result.page_height
      currentPage.value = 0
      editor.value = null
      await nextTick()
      fitWidth()
      await loadAnnotations()
      showToast('已打开', 'success')
    } catch (e) {
      showToast('打开失败: ' + e, 'error')
    } finally {
      loading.value = false
    }
  }
}

async function saveFile() {
  if (!docPath.value) return
  loading.value = true
  loadingText.value = '正在保存...'
  try {
    await invoke('save_as', { input: docPath.value, output: docPath.value })
    showToast('已保存', 'success')
  } catch (e) {
    showToast('保存失败: ' + e, 'error')
  } finally {
    loading.value = false
  }
}

async function saveAsFile() {
  if (!docPath.value) return
  const outputPath = await save({
    filters: [{ name: 'PDF', extensions: ['pdf'] }],
    defaultPath: docPath.value.replace(/\.pdf$/i, '_copy.pdf'),
  })
  if (outputPath) {
    loading.value = true
    loadingText.value = '正在另存为...'
    try {
      await invoke('save_as', { input: docPath.value, output: outputPath })
      showToast('已另存为', 'success')
    } catch (e) {
      showToast('另存为失败: ' + e, 'error')
    } finally {
      loading.value = false
    }
  }
}

async function doDeletePages() {
  if (!docPath.value) return
  const pages = parsePageRange(deletePagesInput.value)
  if (pages.length === 0) {
    showToast('请输入有效页码', 'error')
    return
  }
  loading.value = true
  loadingText.value = '正在删除页面...'
  try {
    const remaining = await invoke<number>('delete_pages', {
      input: docPath.value,
      output: docPath.value,
      pages,
    })
    pageCount.value = remaining
    if (currentPage.value >= remaining) {
      currentPage.value = Math.max(0, remaining - 1)
    }
    showDeleteModal.value = false
    deletePagesInput.value = ''
    await loadAnnotations()
    showToast(`已删除 ${pages.length} 页`, 'success')
  } catch (e) {
    showToast('删除失败: ' + e, 'error')
  } finally {
    loading.value = false
  }
}

async function doExtractPages() {
  if (!docPath.value) return
  const pages = parsePageRange(extractPagesInput.value)
  if (pages.length === 0) {
    showToast('请输入有效页码', 'error')
    return
  }
  const outputPath = await save({
    filters: [{ name: 'PDF', extensions: ['pdf'] }],
    defaultPath: docPath.value.replace(/\.pdf$/i, '_extracted.pdf'),
  })
  if (outputPath) {
    loading.value = true
    loadingText.value = '正在提取页面...'
    try {
      const count = await invoke<number>('extract_pages', {
        input: docPath.value,
        output: outputPath,
        pages,
      })
      showExtractModal.value = false
      extractPagesInput.value = ''
      showToast(`已提取 ${count} 页`, 'success')
    } catch (e) {
      showToast('提取失败: ' + e, 'error')
    } finally {
      loading.value = false
    }
  }
}

async function rotatePage() {
  if (!docPath.value) return
  loading.value = true
  loadingText.value = '正在旋转...'
  try {
    await invoke('rotate_page', {
      input: docPath.value,
      page: currentPage.value,
      rotation: 90,
    })
    await loadAnnotations()
    showToast('已旋转 90°', 'success')
  } catch (e) {
    showToast('旋转失败: ' + e, 'error')
  } finally {
    loading.value = false
  }
}

async function addMergeFile() {
  const selected = await open({
    filters: [{ name: 'PDF', extensions: ['pdf'] }],
    multiple: true,
  })
  if (selected) {
    const files = Array.isArray(selected) ? selected : [selected]
    mergeFiles.value.push(...files)
  }
}

async function doMerge() {
  if (mergeFiles.value.length < 2) {
    showToast('至少需要 2 个文件', 'error')
    return
  }
  const outputPath = await save({
    filters: [{ name: 'PDF', extensions: ['pdf'] }],
    defaultPath: 'merged.pdf',
  })
  if (outputPath) {
    loading.value = true
    loadingText.value = '正在合并...'
    try {
      const sources = mergeFiles.value.map(path => ({ path, pages: [] }))
      const count = await invoke<number>('merge_pdfs', {
        output: outputPath,
        sources,
      })
      showMergeModal.value = false
      mergeFiles.value = []
      showToast(`已合并 ${count} 页`, 'success')
    } catch (e) {
      showToast('合并失败: ' + e, 'error')
    } finally {
      loading.value = false
    }
  }
}

async function compressPdf() {
  if (!docPath.value) return
  const outputPath = await save({
    filters: [{ name: 'PDF', extensions: ['pdf'] }],
    defaultPath: docPath.value.replace(/\.pdf$/i, '_compressed.pdf'),
  })
  if (outputPath) {
    loading.value = true
    loadingText.value = '正在压缩...'
    try {
      const result = await invoke<string>('compress_pdf', {
        input: docPath.value,
        output: outputPath,
        effort: 80,
      })
      showToast('压缩完成: ' + result, 'success')
    } catch (e) {
      showToast('压缩失败: ' + e, 'error')
    } finally {
      loading.value = false
    }
  }
}

function prevPage() {
  if (currentPage.value > 0) {
    currentPage.value--
    editor.value = null
  }
}

function nextPage() {
  if (currentPage.value < pageCount.value - 1) {
    currentPage.value++
    editor.value = null
  }
}

function zoomIn() {
  scale.value = Math.min(scale.value + 0.25, 5.0)
}

function zoomOut() {
  scale.value = Math.max(scale.value - 0.25, 0.25)
}

function fitWidth() {
  if (mainRef.value && pageWidth.value > 0) {
    const containerWidth = mainRef.value.clientWidth - 32
    scale.value = containerWidth / pageWidth.value
  }
}

function onImageLoad() {}

function onImageError(e: Event) {
  console.error('Image load error:', e)
}

watch([currentPage, scale], () => {})

watch(currentPage, () => {
  loadAnnotations()
})
</script>