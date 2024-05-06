"use client";
import React, { useEffect } from "react";
import { Button } from "@nextui-org/react";

import Split from "react-split";
import { Output } from "../output";
import { Editor } from "../editor";

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

const Playground: React.FC = () => {
  const [editorText, setEditorText] = React.useState<string>("");
  const [outputText, setOutputText] = React.useState<string>("");
  const [isLoading, setIsLoading] = React.useState<boolean>(false);
  const [worker, setWorker] = React.useState<Worker | null>(null);
  useEffect(() => {
    window.dispatchEvent(new Event("resize"));
  }, []);

  const stopWorker = () => {
    setIsLoading(false);
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
    setIsLoading(true);
    setOutputText("");
    const webWorker = new Worker(new URL("../../worker.ts", import.meta.url), {
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
    <div className="h-1/2">
      <Split
        gutterAlign="center"
        className="flex h-full"
        sizes={[50, 50]}
        cursor="col-resize"
        direction="horizontal"
        id="content"
        onDragEnd={resizeHandler}
      >
        <Editor editorText={editorText} setEditorText={setEditorText} />
        <Output text={outputText} />
      </Split>
      <Button onClick={startCecile} color="success" isLoading={isLoading}>
        Start
      </Button>
    </div>
  );
};

export default Playground;
