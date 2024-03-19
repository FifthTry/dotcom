function dragElement(element, dragBG) {
    // Reference to the entire window
    const windowElement = element;
    const title= element.querySelector('.header');
    const cornerRB= element.querySelector('.corner-rb')
    const cornerLB= element.querySelector('.corner-lb')
    const cornerL = element.querySelector('.corner-l');
    const cornerR= element.querySelector('.corner-r')
    const cornerB= element.querySelector('.corner-b')
    const minimize= element.querySelector('.minimize')
    const content= element.querySelector('.content')
    let expand = true

    title.addEventListener('mousedown', (e) => {
        let oldBackground = title.style.background;
        let oldBorder = content.style.borderColor;
        title.style.background = dragBG.color;
        content.style.borderColor = dragBG.color;
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
            title.style.background = oldBackground;
            content.style.borderColor = oldBorder;

            document.removeEventListener('mousemove', drag);
            document.removeEventListener('mouseup', mouseup);
        }

        document.addEventListener('mouseup', mouseup);
        document.addEventListener('mousemove', drag);

    });
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
    minimize.addEventListener("click", (e) => {
        e.preventDefault();
        expand = !expand;
        if (expand) {
            content.style.display = "block"
        } else {
            content.style.display = "none"
        }
    })
 }


function closeWindow() {
    // Close the window
    document.getElementById("floating-window").style.display = "none";
}

class CodePreview extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: 'open' });
  }

  connectedCallback() {
    // Create a container div for the shadow DOM content
    const container = document.createElement('div');
    container.setAttribute('id', 'floating-window');
    container.style.position = 'absolute';  // Set the position property to 'absolute' or 'fixed'
    container.style.top = '100px';
    container.style.right = '100px';
    container.style.width = '400px';
    container.style.height = '400px';
    container.style.zIndex = '1000';
    let data = window.ftd.component_data(this);
    let refresh_url = data.refresh.get();
    var iframe_url = data.url.get();
    let dragBG = {color: '#636161'};

    // Set up the HTML content for the shadow DOM
    container.innerHTML = `
      <style>
         #refreshButton:hover {
             background-color: #dcdcdc;
         }
         :host(.dark) #refreshButton:hover {
             background-color: #585656;
         }
      </style>
      <div class="header" style="cursor: move; color: white; background: black; padding: 0 10px; justify-content: space-between; display: flex">
        <div>Preview</div>
        <div class="minimize" style="cursor: pointer">--</div>
      </div>
      <div id="previewComponent" style="display: flex; padding: 5px 5px; gap: 8px; border-width: 0 1px; border-style: solid;">
        <img id="refreshButton" src=${fastn_utils.getStaticValue(refresh_url.get("light"))} style="width: 14px; cursor: pointer; padding: 5px; border-radius: 33%">
        <input type="text" id="urlInput" placeholder="Enter URL">
        <select id="deviceSelect">
            <option value="desktop">Desktop</option>
            <option value="tablet">Tablet</option>
            <option value="mobile">Mobile</option>
        </select>
      </div>
      <div class="content" style="height: calc(100% - 24px); border-width: 1px; border-style: solid; border-color: black; background-color: white">
        <div id="iframe-container" style="height: 100%">
          <!-- You can replace 'your-url-here' with the desired URL -->
          <iframe id="previewFrame" src=${iframe_url} frameborder="0" width="100%" height="100%"></iframe>
        </div>
      </div>
      <div class="corner-rb" style="position: absolute; bottom: 0; right: 0; width: 6px; height: 6px; cursor: nwse-resize"></div>
      <div class="corner-lb" style="position: absolute; bottom: 0; left: 0; width: 6px; height: 6px; cursor: nesw-resize"></div>
      <div class="corner-l" style="position: absolute; left: 0; top: 0; width: 6px; height: calc(100% - 6px); cursor: ew-resize"></div>
      <div class="corner-r" style="position: absolute; right: 0; top: 0; width: 6px; height: calc(100% - 6px); cursor: ew-resize"></div>
      <div class="corner-b" style="position: absolute; left: 6px; bottom: 0; width: calc(100% - 12px); height: 6px; cursor: ns-resize"></div>
    `;

      this.shadowRoot.appendChild(container);
      const webComponent = this;

      const title= container.querySelector('.header');
      const content= container.querySelector('.content');
      const refresh = this.shadowRoot.getElementById('refreshButton');

      let headerColor = function () {
          const darkMode = data.dark_mode.get();
          if (darkMode) {
              webComponent.classList.add("dark");
              title.style.backgroundColor = 'white';
              title.style.color = 'black';
              content.style.borderColor = 'white';
              dragBG.color = '#c5c2c2';
              refresh.src = fastn_utils.getStaticValue(refresh_url.get("dark"));
          } else {
              webComponent.classList.remove("dark");
              title.style.backgroundColor = 'black';
              title.style.color = 'white';
              content.style.borderColor = 'black';
              dragBG.color = '#636161';
              refresh.src = fastn_utils.getStaticValue(refresh_url.get("light"));
          }
      };



      data.dark_mode.on_change(headerColor);

      headerColor();
      dragElement(container, dragBG);

    // Append the container to the shadow DOM
    this.shadowRoot.getElementById('refreshButton').addEventListener('click', () => this.refreshPreview());
    this.shadowRoot.getElementById('deviceSelect').addEventListener('change', () => this.showPreview());
  }

    showPreview() {
        const device = this.shadowRoot.getElementById('deviceSelect').value;
        let width, height;

        switch(device) {
            case 'desktop':
                width = '1024px';
                height = '768px';
                break;
            case 'tablet':
                width = '768px';
                height = '1024px';
                break;
            case 'mobile':
                width = '375px';
                height = '667px';
                break;
            default:
                width = '100%';
                height = '80vh';
        }

        const previewFrame = this.shadowRoot.getElementById('floating-window');
        previewFrame.style.width = width;
        previewFrame.style.height = height;
    }
    refreshPreview() {
        const previewFrame = this.shadowRoot.getElementById('previewFrame');
        previewFrame.src += ''
    }
}

customElements.define('code-preview', CodePreview);
