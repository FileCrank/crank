import {Dispatch, useEffect, useMemo, useState} from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import {Button, Container, Grid, Menu, Select, Stack, Text} from "@mantine/core";
import Format from "./types/Format";
import Dropdown = Menu.Dropdown;

function App() {
    const [formats, setFormats] = useState<Format[]>([]);
    const formatSelect = useMemo(() => formats.map((f) => ({
        value: f.code,
        label: f.name
    })), [formats])

    const [fromFormat, setFromFormat] = useState<Format | undefined>();
    const [toFormat, setToFormat] = useState<Format | undefined>()

    useEffect(() => {
        invoke("get_formats")
            .then((data) => {
                // TODO: do I have to coerce this?
                setFormats(data as Format[])
            })
    }, [])

    return <Container>
        <Text>
            FileCrank
        </Text>
        <form>
            <Stack>
                <Select data={formatSelect} label={"From"} value={fromFormat?.code} onChange={(value) => value && setFromFormat(formats.find((f) => f.code === value))}/>
                <Select data={formatSelect} label={"To"} value={toFormat?.code} onChange={(value) => value && setToFormat(formats.find((f) => f.code === value))} />
                <Button type="submit" disabled={!(fromFormat && toFormat)}>
                    Convert
                </Button>
            </Stack>
        </form>

    </Container>

}

export default App;
