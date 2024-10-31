import React, { memo } from "react";

interface OutputProps {
  text: string;
}

/**
 * Code execution output component
 */
const Output = ({ text }: OutputProps) => (
  <pre
    style={{
      maxHeight: 600,
    }}
    className=" font-monospace text-base ms-1 bg-white overflow-auto "
    // eslint-disable-next-line react/no-danger
    dangerouslySetInnerHTML={{ __html: text }}
    id="output"
  />
);

export default memo(Output);
export type { OutputProps };
