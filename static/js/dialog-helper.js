function closeClosestDialog(element) {
    const dialog = element.closest('dialog')

    if (dialog == null) return

    dialog.close()

    setTimeout(
        () => dialog.remove(),
        10000,
    )
}

document.body.addEventListener('htmx:load', function (event) {
    const element = event.detail.elt

    if (element.nodeName == 'DIALOG') element.showModal()
});
