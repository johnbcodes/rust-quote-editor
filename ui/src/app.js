import "htmx.org";
import * as hyperscript from "hyperscript.org";
hyperscript.browserInit();

// document.addEventListener("turbo:before-frame-render", (event) => {
//     const inputs = event.detail.newFrame.querySelectorAll("input, select, textarea");
//     inputs.forEach(input => {
//         input.addEventListener(
//             "invalid",
//             _event => {
//                 input.classList.add("error");
//             },
//             false
//         );
//     });
// })