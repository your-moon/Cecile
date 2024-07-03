"use client";
import React, { useEffect } from "react";
import { Button, Select, SelectItem } from "@nextui-org/react";

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
    <div className=" p-10 bg-[#D3BEBE]">
      <div className="min-h-12 min-w-40 bg-white my-5 p-2 flex flex-row items-center rounded-xl gap-2">
        <div>Examples</div>
        <Select fullWidth={true} radius="lg" size="sm" className="max-w-56">
          <SelectItem key={1}></SelectItem>
        </Select>
      </div>
      <Split
        gutterAlign="center"
        className="flex min-h-36"
        sizes={[50, 50]}
        cursor="col-resize"
        direction="horizontal"
        id="content"
        onDragEnd={resizeHandler}
      >
        <Editor editorText={editorText} setEditorText={setEditorText} />
        <Output text={outputText} />
      </Split>
      <Button className="mt-5" onClick={startCecile} isLoading={isLoading}>
        Start
      </Button>
    </div>
  );
};

export default Playground;
