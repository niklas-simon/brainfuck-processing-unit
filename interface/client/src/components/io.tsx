import RestService from "@/run-service";
import { Button } from "@nextui-org/button";
import { Textarea } from "@nextui-org/input";
import { Skeleton } from "@nextui-org/skeleton";
import { useEffect, useState } from "react";

const rs = RestService.getInstance();

const inputRequest = rs.getInput();
const outputRequest = rs.getOutput();

export default function IO() {
    const [output, setOutput] = useState<string | null>(null);
    const [input, setInput] = useState<string | null>(null);

    const [isSending, setSending] = useState(false);

    useEffect(() => {
        if (input === null) {
            inputRequest.then(setInput);
        }
        if (output === null) {
            outputRequest.then(setOutput);
        }

        const inputEvent = rs.getInputEvent();
        const onInputMessage = (e: MessageEvent) => {
            setInput(e.data);
        }
        inputEvent.addEventListener("message", onInputMessage);

        const outputEvent = rs.getOutputEvent();
        const onOutputMessage = (e: MessageEvent) => {
            setOutput(e.data);
        }
        outputEvent.addEventListener("message", onOutputMessage);

        return () => {
            inputEvent.removeEventListener("message", onInputMessage);
            outputEvent.removeEventListener("message", onOutputMessage);
        }
    }, []);

    return <div className="flex flex-col gap-4">
        <Skeleton isLoaded={output !== null}>
            <Textarea readOnly label="Output" radius="none"
                className="font-mono" value={output || ""} 
                placeholder="Output from the machine will be displayed here"/>
        </Skeleton>
        <Skeleton isLoaded={input !== null}>
            <Textarea radius="none" label="Input"
                placeholder="Hitting 'Send' updates the input queue to this value" 
                className="font-mono"
                value={input || ""} onChange={e => setInput(e.target.value)}/>
        </Skeleton>
        <div className="flex flex-row gap-4 items-center">
            <Skeleton isLoaded={input !== null} className="rounded-medium">
                <Button variant="light" color="primary" isLoading={isSending} onClick={async () => {
                    setSending(true);
                    await rs.setInput(input || "");
                    setSending(false);
                }}>Send</Button>
            </Skeleton>
            <Skeleton isLoaded={input !== null} className="rounded-medium">
                <Button variant="light" color="danger" isLoading={isSending} onClick={async () => {
                    setSending(true);
                    await rs.setInput("");
                    setSending(false);
                }}>Clear</Button>
            </Skeleton>
        </div>
    </div>
}