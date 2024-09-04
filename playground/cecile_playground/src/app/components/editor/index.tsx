import React from "react";
import AceEditor from "react-ace";

import "ace-builds/src-noconflict/mode-typescript";
import "ace-builds/src-noconflict/theme-tomorrow";
import "ace-builds/src-noconflict/ext-language_tools";

interface EditorProps {
  editorText: string;
  setEditorText: (text: string) => void;
}

const Editor = ({ editorText, setEditorText }: EditorProps) => (
  <AceEditor
    style={{
      minHeight: "500px",
      maxHeight: "600px",
    }}
    className="font-monospace fs-6"
    fontSize={16}
    theme="tomorrow"
    focus
    mode="typescript"
    onChange={setEditorText}
    value={editorText}
  />
);

export { Editor };
export type { EditorProps };
