import RestService from "@/rest-service";
import { Button } from "@nextui-org/button";
import { Textarea } from "@nextui-org/input";
import { Skeleton } from "@nextui-org/skeleton";
import { useEffect, useState } from "react";
import ErrorTooltip from "./error-tooltip";

const rs = RestService.getInstance();

export default function IO() {
    const [output, setOutput] = useState<string | null>(rs.getOutput());
    const [input, setInput] = useState<string | null>(rs.getInput());

    const [isSending, setSending] = useState(false);

    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        const inputProfile = rs.onInputChange(setInput);
        const outputProfile = rs.onOutputChange(setOutput);

        return () => {
            rs.removeListener(inputProfile);
            rs.removeListener(outputProfile);
        }
    }, []);

    return <div className="flex flex-col gap-4">
        <Skeleton isLoaded={output !== null}>
            <Textarea readOnly label="Output" radius="none"
                className="font-mono" value={output || ""} 
                placeholder="Output from the BFPU will be displayed here"/>
        </Skeleton>
        <Skeleton isLoaded={input !== null}>
            <Textarea radius="none" label="Input"
                placeholder="Hitting 'Send' updates the input queue to this value" 
                className="font-mono"
                value={input || ""} onChange={e => setInput(e.target.value)}/>
        </Skeleton>
        <div className="flex flex-row gap-4 items-center">
            <Skeleton isLoaded={input !== null} className="rounded-medium">
                <ErrorTooltip text={error} onClose={() => setError(null)} placement="top-start">
                    <Button variant="light" color="primary" isLoading={isSending} onClick={async () => {
                        setSending(true);
                        await rs.setInput(input || "").catch(e => setError(e.message));
                        setSending(false);
                    }}>Send</Button>
                </ErrorTooltip>
            </Skeleton>
            <Skeleton isLoaded={input !== null} className="rounded-medium">
                <Button variant="light" color="danger" isLoading={isSending} onClick={async () => {
                    setSending(true);
                    await rs.setInput("").catch(e => setError(e.message));
                    setSending(false);
                }}>Clear</Button>
            </Skeleton>
        </div>
    </div>
}