// This editor will need these js/css dependencies
//
// CSS
// Include CodeMirror stylesheet
// https://cdnjs.cloudflare.com/ajax/libs/codemirror/5.63.0/codemirror.min.css
// Include CodeMirror theme (optional)
// https://cdnjs.cloudflare.com/ajax/libs/codemirror/5.63.0/theme/dracula.min.css
//
// JS
// Include CodeMirror library
// https://cdnjs.cloudflare.com/ajax/libs/codemirror/5.63.0/codemirror.min.js
// Include CodeMirror mode (e.g., JavaScript)
// https://cdnjs.cloudflare.com/ajax/libs/codemirror/5.63.0/mode/javascript/javascript.min.js

class CodeEditor extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: 'open' });
  }

  connectedCallback() {
    const shadow = this.shadowRoot;
    const webComponent = this;
    webComponent.style.width = '100%';
    webComponent.style.height = '100%';

    // Import CodeMirror CSS
    const codemirrorCss = document.createElement('link');
    codemirrorCss.rel = 'stylesheet';
    codemirrorCss.href = 'https://cdnjs.cloudflare.com/ajax/libs/codemirror/5.63.0/codemirror.min.css';
    shadow.appendChild(codemirrorCss);

    // Import CodeMirror theme (optional)
    const draculaCss = document.createElement('link');
    draculaCss.rel = 'stylesheet';
    draculaCss.href = 'https://cdnjs.cloudflare.com/ajax/libs/codemirror/5.63.0/theme/dracula.min.css';
    shadow.appendChild(draculaCss);

    // Import CodeMirror library
    const codemirrorJs = document.createElement('script');
    codemirrorJs.src = 'https://cdnjs.cloudflare.com/ajax/libs/codemirror/5.63.0/codemirror.min.js';
    shadow.appendChild(codemirrorJs);

    codemirrorJs.onload = () => {
      // Import CodeMirror mode (e.g., JavaScript)
      const javascriptJs = document.createElement('script');
      javascriptJs.src = 'https://cdnjs.cloudflare.com/ajax/libs/codemirror/5.63.0/mode/javascript/javascript.min.js';
      shadow.appendChild(javascriptJs);
    }

    // Create textarea
    const codeEditor = document.createElement('textarea');
    codeEditor.style.display = "none";
    shadow.appendChild(codeEditor);

    // Wait for CodeMirror library to be loaded before initializing the editor
    codemirrorJs.onload = () => {
      let data = window.ftd.component_data(this);
      let initial_content = fastn_utils.getStaticValue(data.file.get().get('content')).trim();
      let openFile = data.openfile.get();
      if (openFile !== null) {
        initial_content = fastn_utils.getStaticValue(data.openfile.get().get('text').get().get('content'))
      }

      var editor = CodeMirror.fromTextArea(codeEditor, {
        mode: "javascript",
        theme: "dracula",
        lineNumbers: true,
        autofocus: true,
        showCursorWhenSelecting: true,
      })
      editor.setSize("100%", "100%");
      editor.on('change', editor => {
        let content = editor.getValue().trim();
        if (initial_content !== content) {
          data.content.set(content)
        } else {
          data.content.set(null)
        }
      });
      editor.setValue(initial_content);
    };
  }
}

customElements.define('code-editor', CodeEditor);
