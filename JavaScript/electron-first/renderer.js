$ = selector => document.querySelector(selector)

const $count = $('#count')
const $button = $('#button')

$button.addEventListener('click', ()=>{
    const count = +$count.innerText
    $count.innerText = count + 1
})