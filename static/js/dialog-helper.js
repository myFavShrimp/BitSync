// dialog elements

function openDialogModalById(id) {
  const dialog = document.getElementById(id)

  if (dialog == null || dialog.nodeName != 'DIALOG') return

  dialog.showModal()
}

function closeClosestDialog(element) {
  const dialog = element.closest('dialog')

  if (dialog == null || dialog.nodeName != 'DIALOG') return

  dialog.close()
}

function closeClosestDialogAndRemoveElement(element) {
  const dialog = element.closest('dialog')

  if (dialog == null || dialog.nodeName != 'DIALOG') return

  dialog.close()

  // if the same dialog is opened before the removal, id selectors may not work
  dialog.removeAttribute('id')
  dialog.querySelectorAll('[id]').forEach((child) => child.removeAttribute('id'))

  setTimeout(
      () => dialog.remove(),
      1500,
  )
}

// popover

function openPopoverById(id) {
  const popover = document.getElementById(id)

  if (popover == null) return

  popover.showPopover()
}

function closeClosestPopover(element) {
  const dialog = element.closest('[popover]')

  if (dialog == null) return

  dialog.hidePopover()
}

