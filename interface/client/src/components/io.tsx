import { Button } from "@nextui-org/button";
import { Textarea } from "@nextui-org/input";
import { useState } from "react";

export default function IO() {
    const [output, setOutput] = useState("");
    const [input, setInput] = useState("");

    return <div className="flex flex-col gap-4">
        <Textarea readOnly label="Output" radius="none"
            className="font-mono" value={output} 
            placeholder="Output from the machine will be displayed here"/>
        <Textarea radius="none" label="Input"
            placeholder="Will be appended to the input queue upon pressing 'Send'" 
            className="font-mono"
            value={input} onChange={e => setInput(e.target.value)}/>
        <div className="flex flex-row gap-4 items-center">
            <Button variant="light" color="primary" className="w-min" onClick={() => {
                setInput("");
                window.dispatchEvent(new CustomEvent("sendInput", {
                    detail: input
                }))
            }}>Send</Button><Button variant="light" color="danger" className="w-min" onClick={() => {
                setOutput("");
                setInput("");
            }}>Clear</Button>
        </div>
    </div>
}