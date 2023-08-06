var esto = "prometo"
var promesa = "esperarte bun"
var pinche = "pinche wsl lo amo"
function a() {
    console.log(esto)
    console.log(promesa)
}
function b() {
    setTimeout(() => {
        console.log(esto);
    }, 1000)
    console.log(promesa);
}
async function c() {
    await new Promise(resolve => {
        setTimeout(() => {
            console.log(esto);
            resolve()
        }, 1000)
    })
    console.log(promesa);
}
function main() {
    a()
    b()
    c()
}
main()