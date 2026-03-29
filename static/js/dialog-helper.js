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

    setTimeout(
        () => dialog.remove(),
        10000,
    )
}

function openMoveModal(dialogId, action, path) {
    const dialog = document.getElementById(dialogId)

    if (dialog == null || dialog.nodeName != 'DIALOG') return

    const form = dialog.querySelector('form')
    form.hsFetch.resource(action)
    form.querySelector('input[name="destination_path"]').value = path

    dialog.showModal()
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

