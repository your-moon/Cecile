"use client";
import React, { useEffect } from "react";
import {
  Card,
  CardHeader,
  CardBody,
  CardFooter,
  Divider,
  Link,
  Image,
  Button,
} from "@nextui-org/react";
import AceEditor from "react-ace";
import { Output } from "./output";

import "ace-builds/src-noconflict/mode-typescript";
import "ace-builds/src-noconflict/theme-tomorrow";
import "ace-builds/src-noconflict/ext-language_tools";
import Split from "react-split";

type CecileOutMessageOutput = {
  type: "Output";
  text: string;
};

type CecileOutMessageExitFailure = {
  type: "ExitFailure";
};

type CecileOutMessageExitSuccess = {
  type: "ExitSuccess";
};

type CecileOutMessage =
  | CecileOutMessageOutput
  | CecileOutMessageExitFailure
  | CecileOutMessageExitSuccess;

export default function Playground() {
  const [editorText, setEditorText] = React.useState<string>("");
  const [outputText, setOutputText] = React.useState<string>("");
  const [worker, setWorker] = React.useState<Worker | null>(null);
  useEffect(() => {
    window.dispatchEvent(new Event("resize"));
  }, []);

  const stopWorker = () => {
    setWorker((currentWorker) => {
      if (currentWorker) {
        currentWorker.terminate();
      }
      return null;
    });
  };

  const addOutputText = (text: string) => {
    setOutputText((currentOutputText) => currentOutputText + text);
  };
  const startCecile = () => {
    stopWorker();
    setOutputText("");
    const webWorker = new Worker(new URL("../worker.ts", import.meta.url), {
      type: "module",
    });

    webWorker.onmessage = (event) => {
      console.log(event.data);
      const msg: CecileOutMessage = JSON.parse(
        event.data as string,
      ) as CecileOutMessage;

      switch (msg.type) {
        case "Output":
          addOutputText(msg.text);
          break;
        case "ExitSuccess":
          stopWorker();
          addOutputText("---\nProgram exited successfully.\n");
          break;
        case "ExitFailure":
          stopWorker();
          addOutputText("---\nProgram exited with errors.\n");
          break;
        default:
          break;
      }
    };
    webWorker.postMessage(editorText);
    setWorker(webWorker);
  };
  const resizeHandler = () => window.dispatchEvent(new Event("resize"));

  return (
    <>
      <Split
        gutterAlign="center"
        className="flex "
        sizes={[50, 50]}
        cursor="col-resize"
        direction="horizontal"
        id="content"
        onDragEnd={resizeHandler}
      >
        <AceEditor
          className="h-100 font-monospace fs-6"
          theme="tomorrow"
          focus
          mode="typescript"
          onChange={setEditorText}
          value={editorText}
        />
        <Output text={outputText} />
      </Split>

      <Button onClick={startCecile}>Start</Button>
    </>
  );
}
