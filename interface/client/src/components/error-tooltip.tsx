import { Button } from "@nextui-org/button";
import { Tooltip } from "@nextui-org/tooltip";
import { ReactNode } from "react";
import { X } from "react-feather";

export default function ErrorTooltip({text, onClose, children, placement}: {
        text: string | null,
        onClose: () => void,
        children: ReactNode | ReactNode[] | string,
        placement?: "top-start" | "top" | "top-end" | "bottom-start" | "bottom" | "bottom-end" | "left-start" | "left" | "left-end" | "right-start" | "right" | "right-end"}) {
    return <Tooltip 
        content={
            <div className="flex flex-row gap-4 items-center">
                <span className="text-danger">{text}</span>
                <Button onClick={onClose} isIconOnly variant="light" size="sm">
                    <X/>
                </Button>
            </div>
        }
        showArrow isOpen={text != null} placement={placement}
    >
           {children} 
    </Tooltip>
}