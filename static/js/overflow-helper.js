function createHorizontalOverflowHandler(element, overflowLeftClass, overflowRightClass) {
  const wrapper = element.parentElement

  const update = () => {
    element.scrollLeft > 0
      ? wrapper.classList.add(overflowLeftClass)
      : wrapper.classList.remove(overflowLeftClass)

    element.scrollLeft + element.clientWidth < element.scrollWidth - 1
      ? wrapper.classList.add(overflowRightClass)
      : wrapper.classList.remove(overflowRightClass)
  }

  new ResizeObserver(update).observe(element)

  return update
}
