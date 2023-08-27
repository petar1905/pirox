import init, { calculate } from "../../pkg/pirox.js"

init().then(() => {
    alert(calculate("9/3*7+5/5-6"))
})