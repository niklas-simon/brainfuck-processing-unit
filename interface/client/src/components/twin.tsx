import { ChevronDown, ChevronUp } from "react-feather";
import Tape from "./tape";
import { Card, CardBody } from "@nextui-org/card";

export default function Twin() {
    return <div className="w-full h-full flex flex-col items-center justify-center overflow-hidden">
        <Card className="bg-content2 overflow-visible" radius="none">
            <CardBody className="flex flex-col items-center w-[256px] overflow-visible">
                <ChevronDown/>
                <Tape data={Array.from(new Array(7)).map((_, i) => {
                    return {
                        address: i,
                        value: [">", "<", "+", "-", ".", ",", "[", "]"][Math.floor(Math.random() * 8)]
                    }
                })}/>
                <Tape className="mt-4" data={Array.from(new Array(7)).map((_, i) => {
                    return {
                        address: i,
                        value: Math.floor(Math.random() * 256)
                    }
                })}/>
                <ChevronUp/>
            </CardBody>
        </Card>
    </div>
}