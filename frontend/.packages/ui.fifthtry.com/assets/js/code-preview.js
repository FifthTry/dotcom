function addExpandable(device) {
    console.log("device", device);
    let component = document.getElementById("preview-container");
    if (component == null || device.get() !== 0) {
        return false;
    }
    expand(component);
    return true;
}


function addMover(create, edit, preview, current_file, openfile) {
    let component = document.getElementById("preview-container");
    if (component !== null && preview.get() && !create.get() && !edit.get() && ((openfile.get() !== null && openfile.get().get("filetype").get() !== "image") || (openfile.get() === null && current_file.get().get("filetype").get() !== "image"))) {
        dragElement(component);
        return false;
    }
    return true;
}

function dragElement(element) {
    // Reference to the entire window
    const windowElement = element;
    const title= document.getElementById("header")

    title.addEventListener('mousedown', (e) => {
        // Initialize variables for tracking drag state
        let offsetX = e.clientX - windowElement.getBoundingClientRect().left;
        let offsetY = e.clientY - windowElement.getBoundingClientRect().top;

        const drag = (e) => {
            e.preventDefault()

            const newX = e.clientX - offsetX;
            const newY = e.clientY - offsetY;

            windowElement.style.left = `${newX}px`;
            windowElement.style.top = `${newY}px`;
        }
        const mouseup = () => {
            document.removeEventListener('mousemove', drag);
            document.removeEventListener('mouseup', mouseup);
        }

        document.addEventListener('mouseup', mouseup);
        document.addEventListener('mousemove', drag);

    });
}

function expand(element) {
    // Reference to the entire window
    const windowElement = element;
    const cornerRB= document.getElementById("corner-rb")
    const cornerLB= document.getElementById("corner-lb")
    const cornerL= document.getElementById("corner-l")
    const cornerR= document.getElementById("corner-r")
    const cornerB= document.getElementById("corner-b")

    cornerRB.addEventListener('mousedown', (e) => {
        let offsetX = e.clientX - windowElement.clientWidth;
        let offsetY = e.clientY - windowElement.clientHeight;

        const drag = (e) => {
            e.preventDefault()

            const newX = e.clientX - offsetX;
            const newY = e.clientY - offsetY;

            windowElement.style.width = `${newX}px`;
            windowElement.style.height = `${newY}px`;
        }
        const mouseup = () => {
            document.removeEventListener('mousemove', drag);
            document.removeEventListener('mouseup', mouseup);
        }

        document.addEventListener('mouseup', mouseup);
        document.addEventListener('mousemove', drag);

    });
    cornerLB.addEventListener('mousedown', (e) => {
        let prevX = e.clientX;
        let prevY = e.clientY;

        const drag = (e) => {
            let rect = windowElement.getBoundingClientRect();
            e.preventDefault()

            windowElement.style.width = rect.width + (prevX - e.clientX) + "px";
            windowElement.style.height = rect.height - (prevY - e.clientY) + "px";
            windowElement.style.left = rect.left - (prevX - e.clientX) + "px";

            prevX = e.clientX;
            prevY = e.clientY;
        }
        const mouseup = () => {
            document.removeEventListener('mousemove', drag);
            document.removeEventListener('mouseup', mouseup);
        }

        document.addEventListener('mouseup', mouseup);
        document.addEventListener('mousemove', drag);

    });
    cornerL.addEventListener('mousedown', (e) => {
        let prevX = e.clientX;
        const drag = (e) => {
            let rect = windowElement.getBoundingClientRect();
            e.preventDefault()

            windowElement.style.width = rect.width + (prevX - e.clientX) + "px";
            windowElement.style.left = rect.left - (prevX - e.clientX) + "px";

            prevX = e.clientX;
        }
        const mouseup = () => {
            document.removeEventListener('mousemove', drag);
            document.removeEventListener('mouseup', mouseup);
        }

        document.addEventListener('mouseup', mouseup);
        document.addEventListener('mousemove', drag);

    });
    cornerR.addEventListener('mousedown', (e) => {
        let offsetX = e.clientX - windowElement.clientWidth;
        const drag = (e) => {
            e.preventDefault();
            const newX = e.clientX - offsetX;
            windowElement.style.width = `${newX}px`;
        }
        const mouseup = () => {
            document.removeEventListener('mousemove', drag);
            document.removeEventListener('mouseup', mouseup);
        }

        document.addEventListener('mouseup', mouseup);
        document.addEventListener('mousemove', drag);

    });
    cornerB.addEventListener('mousedown', (e) => {
        let offsetY = e.clientY - windowElement.clientHeight;
        const drag = (e) => {
            e.preventDefault()
            const newY = e.clientY - offsetY;
            windowElement.style.height = `${newY}px`;
        }
        const mouseup = () => {
            document.removeEventListener('mousemove', drag);
            document.removeEventListener('mouseup', mouseup);
        }

        document.addEventListener('mouseup', mouseup);
        document.addEventListener('mousemove', drag);

    });
}


function getDeviceWidth(devices, idx) {
    return devices.get(idx.get()).item.get("width").get();
}

function getDeviceHeight(devices, idx) {
    return devices.get(idx.get()).item.get("height").get();
}

document.addEventListener('DOMContentLoaded', () => {
    dragElement(document.getElementById("preview-container"));
    addExpandable(fastn.mutable(1))
});
