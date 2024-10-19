import IO from "@/components/io";
import DefaultLayout from "@/layouts/default";
import { Card, CardBody } from "@nextui-org/card";

export default function IOPage() {
    return <DefaultLayout>
        <Card radius="none">
            <CardBody>
                <IO/>
            </CardBody>
        </Card>
    </DefaultLayout>
}