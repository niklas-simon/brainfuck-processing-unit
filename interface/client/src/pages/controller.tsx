import Controller from "@/components/controller";
import DefaultLayout from "@/layouts/default";
import { Card, CardBody } from "@nextui-org/card";

export default function ControllerPage() {
    return <DefaultLayout>
        <Card radius="none">
            <CardBody>
                <Controller/>
            </CardBody>
        </Card>
    </DefaultLayout>
}