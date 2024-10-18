import Twin from "@/components/twin";
import DefaultLayout from "@/layouts/default";
import { Card, CardBody } from "@nextui-org/react";

export default function TwinPage() {
    return <DefaultLayout>
        <Card radius="none">
            <CardBody>
                <Twin/>
            </CardBody>
        </Card>
    </DefaultLayout>
}