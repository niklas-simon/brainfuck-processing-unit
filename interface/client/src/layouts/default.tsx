import { Dropdown, DropdownTrigger, DropdownMenu, DropdownItem } from "@nextui-org/dropdown";
import { Link } from "@nextui-org/link";
import { Navbar, NavbarBrand, NavbarContent, NavbarItem } from "@nextui-org/navbar";
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

    return (
        <div className="min-w-screen min-h-screen flex flex-col items-center">
            <Navbar isBordered>
                <NavbarBrand>
                    <Link className="flex flex-row gap-4" color="foreground" href="/">
                        <img src="dhge_logo.png" className="h-[2em]"/>
                        <p className="font-bold text-inherit">Brainfuck Interpreter</p>
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
        </div>
    );
}
