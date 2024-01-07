// chakra-theme.js

import { extendTheme } from "@chakra-ui/react";

const theme = extendTheme({
    styles: {
        global: (props) => ({
            body: {
                bg: props.colorMode === "dark" ? "#253746" : "#D9E1E2",
            },
        }),
    },
});

export default theme;

