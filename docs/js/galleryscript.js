let info_index = -1;
let move_direction = -1;
let x = 0;
const fps = 60;
const display_speed = 4;

function display_info(gallery_num) {
    let info = document.getElementById("gallery-info"+gallery_num);
    info.style.display = "block";
    info_index = gallery_num;  

    fetch("gallery/gallery"+gallery_num+"-save.txt")
        .then((res) => res.text())
        .then((text) => {
            let lines = text.split("\n");
            let contents = "";
            contents += lines[1] + "<br>";
            contents += lines[2] + "<br><br>";
            contents += "click to download"
            info.innerHTML = contents;
        })
        .catch((e) => console.error(e));

    document.getElementById("gallery-img"+gallery_num).style.zIndex = "2";
    document.getElementById("gallery-info"+gallery_num).style.zIndex = "1";

    if (gallery_num % 2 == 0) {
        info.style.left = "50%";
        move_direction = -1;
        x = 50;
    } else {
        info.style.left = "2%";
        move_direction = 1;
        x = 2;
    }
}

function close_info(gallery_num) {
    let info = document.getElementById("gallery-info"+gallery_num);
    info.innerHTML = "";
    info.style.display = "none";
    info_index = -1;

    document.getElementById("gallery-img"+gallery_num).style.zIndex = "0";
    document.getElementById("gallery-info"+gallery_num).style.zIndex = "-1";
}

window.setInterval(function() {
    if (info_index == -1) {
        return;
    }

    x = x + move_direction * display_speed;
    if (move_direction == -1) {
        if (x <= 2) {
            x = 2;
        }
    } else {
        if (x >= 50) {
            x = 50;
        }
    }

    document.getElementById("gallery-info"+info_index).style.left = x + "%";
}, (1/fps)*1000);