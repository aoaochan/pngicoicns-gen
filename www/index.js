import init, { generate, set_panic_hook } from "https://cdn.jsdelivr.net/gh/aoaochan/pngicoicns-gen@latest/pkg/rswasm_icongen.js";

const imgInput = document.getElementById("imginput");
const linksdiv = document.getElementById("links");
const btnchoose = document.getElementById("choose-file-btn");

async function main() {
  await init("https://cdn.jsdelivr.net/gh/aoaochan/pngicoicns-gen@latest/pkg/rswasm_icongen_bg.wasm");
  set_panic_hook();

  btnchoose.addEventListener("click", () => {
    imgInput.click();
  });

  imgInput.addEventListener("change", (event) => {
    const file = event.target.files[0];
    const reader = new FileReader();

    reader.onload = (e) => {
      const bytes = new Uint8Array(e.target.result);
      
      let imgs = generate(bytes);
      let j = 0;

      const promises = imgs.map((img, i) => {
        let type = "image/png";
        let extension = "png";
        let filename = "icon";
        if (i === 0) {
          type = "image/x-icon";
          extension = "ico";
        } else if (i === 1) {
          type = "image/icns";
          extension = "icns";
        } else {
          switch (j) {
            case 0:
              filename = "32x32";
              break;
            case 1:
              filename = "64x64";
              break;
            case 2:
              filename = "128x128";
              break;
            case 3:
              filename = "128x128@2x";
              break;
            case 4:
              filename = "icon";
              break;
            default:
              filename = `icon-${j}`;
              break;
          }
          j++;
        }

        const blob = new Blob([img], { type });
        const url = URL.createObjectURL(blob);
        
        return new Promise((resolve) => {
          fetch(url)
            .then(res => res.blob())
            .then(file => {
              let a = document.createElement('a');
              a.id = "dl";
              linksdiv.appendChild(a);
              a.href = URL.createObjectURL(file);
              a.setAttribute('download', `${filename}.${extension}`);
              a.click();

              resolve();
            })
            .catch(console.error);
        });
      });

      Promise.all(promises).catch(console.error);

      linksdiv.innerHTML = ""; // Clear previous links
    };

    reader.readAsArrayBuffer(file);
  });
}

main();