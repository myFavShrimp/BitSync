function closeNextDialog(el) {
    const dialog = el.closest('dialog')

    dialog.close()

    setTimeout(
        () => dialog.remove(),
        10000,
    )
}
