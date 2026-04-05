function initDropUpload(dropZone, form, activeClass) {
  let dragoverTimeout

  document.addEventListener('dragover', (event) => {
    event.preventDefault()
    dropZone.classList.add(activeClass)
    clearTimeout(dragoverTimeout)
    dragoverTimeout = setTimeout(() => dropZone.classList.remove(activeClass), 500)
  })

  document.addEventListener('drop', (event) => {
    event.preventDefault()
    clearTimeout(dragoverTimeout)
    dropZone.classList.remove(activeClass)

    if (dropZone.contains(event.target)) {
      const fileInput = form.querySelector('input[type="file"]')
      fileInput.files = event.dataTransfer.files
      fileInput.dispatchEvent(new Event('change'))
    }
  })
}
