function getPopoverTrigger(element) {
  var popover = element.closest('[popover]')
  return document.querySelector('[popovertarget="' + popover.id + '"]')
}

function handleButtonLoading(button, fetchable, loadingClass, delayMs) {
  if (fetchable.state() === 'pending') {
    button.disabled = true

    button._loadingTimeout = setTimeout(
      () => button.classList.add(loadingClass),
      delayMs,
    )
  } else {
    clearTimeout(button._loadingTimeout)
    button.classList.remove(loadingClass)
    button.disabled = false
  }
}
