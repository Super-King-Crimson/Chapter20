let count = 0;
let maxCount = 0;

let countText = document.getElementById("count");
let maxCountText = document.getElementById("maxcount");

function updateText() {
    countText.innerHTML = `Count: ${count}`;
    maxCountText.innerHTML = `Max Count: ${maxCount}`;
}

function inc() {
    count++;
    updateText();
}

function dec() {
    count--;
    updateText();
}

function reset() {
    if (count > maxCount) {
        maxCount = count;
    }

    count = 0;
    updateText();
}