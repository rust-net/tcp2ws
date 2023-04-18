declare module '*.html' { // handle by file-loader
    const value: string;
    export default value;
}

declare module '*.css' { // handle by css-loader
    const value: any;
    export default value;
}

// Need to add dependencies:
// $ yarn add file-loader
declare module '!file-loader!*' {
    const value: string;
    export default value; // return a path
}

// $ yarn add raw-loader
declare module '!raw-loader!*' {
    const value: string;
    export default value; // return the file content
}
