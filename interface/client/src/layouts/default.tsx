import ErrorTooltip from "@/components/error-tooltip";
import { Dropdown, DropdownTrigger, DropdownMenu, DropdownItem } from "@nextui-org/dropdown";
import { Link } from "@nextui-org/link";
import { Navbar, NavbarBrand, NavbarContent, NavbarItem } from "@nextui-org/navbar";
import { useEffect, useState } from "react";
import { Menu } from "react-feather";
import { useLocation } from "react-router-dom";

const links = [
    {
        name: "Home",
        link: "/"
    },
    {
        name: "Programmer",
        link: "/programmer"
    },
    {
        name: "IO",
        link: "/io"
    },
    {
        name: "Controller",
        link: "/controller"
    },
    {
        name: "Digital Twin",
        link: "/twin"
    }
]

export default function DefaultLayout({
    children,
}: {
    children: React.ReactNode;
}) {
    const location = useLocation();

    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        const handler = (e: Event) => {
            const event = e as CustomEvent;
            if (event.detail instanceof Event) {
                setError("unknown event error occured");
            } else {
                setError(event.detail.message);
            }
        };

        window.addEventListener("requestError", handler);

        return () => window.removeEventListener("requestError", handler);
    }, []);

    return (
        <div className="min-w-screen min-h-screen flex flex-col items-center">
            <Navbar isBordered>
                <NavbarBrand>
                    <Link className="flex flex-row" color="foreground" href="/">
                        <img src="icon.png" className="h-[2em] mr-2"/>
                        <img src="dhge_logo_writing.png" className="h-[2em] mr-4"/>
                        <p className="font-bold text-inherit">BFPU Control Server</p>
                    </Link>
                </NavbarBrand>
                <NavbarContent className="hidden md:flex lg:hidden gap-4" justify="end">
                    {links.map(link => <NavbarItem key={link.link}>
                        <Link href={link.link} color={location.pathname === link.link ? "primary" : "foreground"}>
                            {link.name}
                        </Link>
                    </NavbarItem>)}
                </NavbarContent>
                <NavbarContent className="flex md:hidden gap-4" justify="end">
                    <Dropdown>
                        <DropdownTrigger>
                            <NavbarItem>
                                <Menu/>
                            </NavbarItem>
                        </DropdownTrigger>
                        <DropdownMenu>
                            {links.map(link => <DropdownItem textValue={link.name} key={link.link}>
                            <Link href={link.link} color={location.pathname === link.link ? "primary" : "foreground"}>
                                {link.name}
                            </Link>
                        </DropdownItem>)}
                        </DropdownMenu>
                    </Dropdown>
                </NavbarContent>
            </Navbar>
            <div className="flex-1 flex flex-col gap-4 p-4 max-w-[1024px] w-full">
                {children}
            </div>
            <ErrorTooltip placement="top-end" text={error} onClose={() => setError(null)}>
                <div className="absolute bottom-0 right-0"></div>
            </ErrorTooltip>
        </div>
    );
}
