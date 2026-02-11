
import {createContext, ReactNode, useEffect} from "react";

const ThemeContext = createContext<undefined>(undefined);

export function ThemeProvider({children}: { children: ReactNode }) {
    useEffect(() => {
        document.documentElement.classList.remove("light");
        document.documentElement.classList.add("dark");
    }, []);

    return (
        <ThemeContext.Provider value={undefined}>
            {children}
        </ThemeContext.Provider>
    );
}
