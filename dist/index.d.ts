#!/usr/bin/env node
declare global {
    var logln: (message: string) => void;
    var log: (message: string) => void;
    var log_with_color: (message: string, color: string) => void;
    var read_file: (file: string) => string;
    var canonicalize: (file: string) => string;
    var file_exists: (file: string) => boolean;
    var create_dir_all: (dir: string) => void;
    var write_file: (file: string, content: string) => void;
}
export {};
