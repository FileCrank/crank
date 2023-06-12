import {Dispatch, useEffect, useMemo, useState} from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import {Button, Container, FileInput, Grid, Menu, Select, Stack, Text} from "@mantine/core";
import Format from "./types/Format";
import {File} from "buffer";


function convert(from: Format, to: Format, file: File): Promise<Blob> {
    return new Promise((resolve, reject) => {
        invoke("convert_file", {
            from,
            to,
            // TODO: need to either a) get a real filepath not the browser File, or b) read the whole thing in ahead of time. Neither is ideal
            sourceFile: "/Users/will/Documents/cranktest.txt"
        }).then((data) => {
            console.log("Got convert_file data", data);
        }).catch((err) => {
            console.error("convert_file error", err);
        })
    })
}

function App() {
    const [formats, setFormats] = useState<Format[]>([]);
    const formatSelect = useMemo(() => formats.map((f) => ({
        value: f.code,
        label: f.name
    })), [formats])

    const [fromFormat, setFromFormat] = useState<Format | undefined>();
    const [toFormat, setToFormat] = useState<Format | undefined>();
    const [file, setFile] = useState<File | undefined>();

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
                <FileInput label={"File"} placeholder={"Pick File"} onChange={(payload) => payload && setFile(payload)}/>
                <Button type="submit" disabled={!(fromFormat && toFormat && file)} onClick={(event) => {
                    event.preventDefault();
                    if (fromFormat && toFormat && file) {
                        convert(fromFormat, toFormat, file);
                    }
                }}>
                    Convert
                </Button>
            </Stack>
        </form>

    </Container>

}

export default App;
