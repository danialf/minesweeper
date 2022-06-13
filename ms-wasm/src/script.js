import init, {
    getState,
    main,
    openField,
    toggleFlag,
  } from "../pkg/minesweeper.js";

  async function run() {
    await init();
    main();
    render();
  }

  function render() {
    let root = document.getElementById("root");
    root.innerHTML = "";

    let data = getState()
      .split("\n")
      .map((row) => row.trim().split(/\s+/));
    root.style.display = "inline-grid";
    root.style.gridTemplate = `repeat(${data.length}, auto) / repeat(${data[0].length}, auto)`;

    for (let x = 0; x < data.length; x++) {
      for (let y = 0; y < data[x].length; y++) {
        let element = document.createElement("a");
        element.classList.add("field");
        element.href = "#";
        element.innerText = data[x][y];

        element.addEventListener("click", (evt) => {
          evt.preventDefault();

          openField(x, y);
          render();
        });

        element.addEventListener("contextmenu", (evt) => {
          evt.preventDefault();

          toggleFlag(x, y);
          render();
        });

        root.appendChild(element);
      }
    }
  }

  run();