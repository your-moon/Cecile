"use client";
import React, { memo, useEffect } from "react";
import { Button, Card, Select, SelectItem } from "@nextui-org/react";

import Split from "react-split";
import { Output } from "../output";
import { Editor } from "../editor";
import { examples } from "../examples/examples";

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
    <div className="bg-[#dad7cd] h-full w-full flex flex-row items-center justify-center">
      <div className="m-10 w-4/5  rounded-lg bg-white">
        <div className="min-w-40 bg-slate-50 px-4 h-20 flex flex-row items-center rounded-t-lg gap-2 justify-between">
          <div>Examples</div>
          <Select
            fullWidth={true}
            radius="lg"
            size="md"
            className="max-w-56"
            defaultSelectedKeys={["1"]}
            onChange={(e) => {
              console.log(e.target.value);
              const example = examples.find(
                (example) => example.key === e.target.value,
              );
              if (example) {
                setEditorText(example.code);
              }
            }}
          >
            {examples.map((example) => (
              <SelectItem key={example.key || ""} value={example.key}>
                {example.name}
              </SelectItem>
            ))}
          </Select>
        </div>
        <Split
          gutterAlign="center"
          className="flex"
          sizes={[50, 50]}
          cursor="col-resize"
          direction="horizontal"
          id="content"
          onDragEnd={resizeHandler}
        >
          <Editor editorText={editorText} setEditorText={setEditorText} />
          <Output text={outputText} />
        </Split>
        <div className="flex bg-slate-50 items-center justify-end rounded-br-lg rounded-bl-lg">
          <Button
            className="m-3 bg-white"
            onClick={startCecile}
            isLoading={isLoading}
          >
            Start
          </Button>
        </div>
      </div>
    </div>
  );
};

export default Playground;
