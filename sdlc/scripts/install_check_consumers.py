"""Installed package consumers. Inputs are files, never source substitutions."""

SOURCES = {
    "python": '''import json, pathlib, sys
import thinkthen
root = pathlib.Path(sys.argv[1])
call = thinkthen.Engine(replay=str(root / "recording"), cache=False).decide(
    (root / "question.txt").read_text(), (root / "report.txt").read_text())
print(json.dumps({"value": call.value, "requests_sent": call.facts["requests_sent"]}))
''',
    "node": '''const fs = require('node:fs');
const {Engine} = require('thinkthen');
const root = process.argv[2];
(async () => {
  const call = await new Engine({replay: root + '/recording', cache: false}).decide(
    fs.readFileSync(root + '/question.txt', 'utf8'), fs.readFileSync(root + '/report.txt', 'utf8'));
  console.log(JSON.stringify({value: call.value, requests_sent: call.facts["requests_sent"]}));
})().catch(error => { console.error(error.message); process.exitCode = 1; });
''',
    "ruby": '''require 'json'
require 'thinkthen'
root = ARGV.fetch(0)
call = ThinkThen::Engine.new(replay: File.join(root, 'recording'), cache: false).decide(
  File.read(File.join(root, 'question.txt')), File.read(File.join(root, 'report.txt')))
puts JSON.generate(value: call.value, requests_sent: call.facts.fetch(:requests_sent))
''',
    "rust": '''use std::{env, fs};
use thinkthen::{Answer, EngineBuilder, Question};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = std::path::PathBuf::from(env::args().nth(1).ok_or("sample argument")?);
    let engine = EngineBuilder::from_env()?.replay(root.join("recording"))?.build()?;
    let question = Question::decide(&fs::read_to_string(root.join("question.txt"))?)?.cut();
    let call = engine.decide(&question, &fs::read_to_string(root.join("report.txt"))?)?;
    let answer = match call.value() { Answer::Yes => "true", Answer::No => "false", Answer::Unsure => "null" };
    println!("{{\\\"value\\\":{},\\\"requests_sent\\\":{}}}", answer, call.facts().requests_sent());
    Ok(())
}
''',
    "c": '''#include <stdio.h>
#include <stdlib.h>
#include <thinkthen.h>
static char *read_file(const char *path) {
    FILE *f = fopen(path, "rb");
    if (!f || fseek(f, 0, SEEK_END)) return NULL;
    long n = ftell(f);
    if (n < 0 || fseek(f, 0, SEEK_SET)) { fclose(f); return NULL; }
    char *text = malloc((size_t)n + 1);
    if (!text || fread(text, 1, (size_t)n, f) != (size_t)n) { fclose(f); free(text); return NULL; }
    text[n] = 0; fclose(f); return text;
}
int main(int argc, char **argv) {
    if (argc == 2) { printf("%d.%d.%d\\n", THINKTHEN_VERSION_MAJOR, THINKTHEN_VERSION_MINOR, THINKTHEN_VERSION_PATCH); return 0; }
    if (argc != 3) return 1;
    char *settings = read_file(argv[1]), *request = read_file(argv[2]);
    if (!settings || !request) return 1;
    thinkthen_engine *engine = thinkthen_engine_new_with(settings);
    if (!engine) return 1;
    char *call = thinkthen_call(engine, request);
    if (!call) { thinkthen_engine_free(engine); return 1; }
    puts(call); thinkthen_free_string(call); thinkthen_engine_free(engine);
    free(settings); free(request); return 0;
}
''',
    "csharp": '''using System;
using System.IO;
using ThinkThen;
using Engine engine = Engine.Open(File.ReadAllText(args[0]));
Console.WriteLine(engine.Call(File.ReadAllText(args[1])));
''',
    "java": '''import java.nio.file.Files;
import java.nio.file.Path;
import thinkthen.Door;
public class InstallCheck {
  public static void main(String[] args) throws Exception {
    try (Door door = new Door(Files.readString(Path.of(args[0])))) {
      System.out.println(door.call(Files.readString(Path.of(args[1]))));
    }
  }
}
''',
    "dart": '''import 'dart:convert';
import 'dart:io';
import 'package:thinkthen_dart/thinkthen_dart.dart';
void main(List<String> args) {
  final door = Door(args[0]);
  final engine = door.create(File(args[1]).readAsStringSync());
  try {
    print(jsonEncode(door.call(engine, File(args[2]).readAsStringSync())));
  } finally { door.engineFree(engine); }
}
''',
    "php": '''<?php
require __DIR__ . '/vendor/autoload.php';
$engine = new ThinkThen($argv[1], file_get_contents($argv[2]));
try { echo $engine->call(file_get_contents($argv[3])), "\\n"; }
finally { $engine->close(); }
''',
    "go": '''package main
import (
 "context"
 "fmt"
 "os"
 thinkthen "github.com/botassembly/thinkthen/libraries/go"
)
func main() {
 settings, err := os.ReadFile(os.Args[1]); if err != nil { panic(err) }
 engine, err := thinkthen.NewWith(string(settings)); if err != nil { panic(err) }
 defer engine.Close()
 request, err := os.ReadFile(os.Args[2]); if err != nil { panic(err) }
 call, err := engine.Call(context.Background(), string(request)); if err != nil { panic(err) }
 fmt.Println(call)
}
''',
    "r": '''args <- commandArgs(trailingOnly = TRUE)
library(thinkthen)
tt_engine(replay = file.path(args[1], "recording"), cache = FALSE)
question <- readChar(file.path(args[1], "question.txt"), file.info(file.path(args[1], "question.txt"))$size)
text <- readChar(file.path(args[1], "report.txt"), file.info(file.path(args[1], "report.txt"))$size)
stopifnot("thinkthen" %in% names(getLoadedDLLs()))
call <- tt_decide(question, text)
stopifnot(identical(call$value, TRUE), call$facts$requests_sent == 0)
miss <- tryCatch(tt_decide(question, paste0(text, " Replay miss.")), thinkthen_local = identity)
stopifnot(inherits(miss, "thinkthen_local"), identical(miss$retryable, FALSE),
          identical(conditionMessage(miss), "the replay folder holds no answer for this question"))
value <- if (identical(call$value, TRUE)) "true" else if (identical(call$value, FALSE)) "false" else "null"
cat('{"value":', value, ',"requests_sent":', call$facts$requests_sent, '}\\n', sep = "")
''',
}


def write_consumer(folder, language, name):
    path = folder / name
    path.write_text(SOURCES[language], encoding="utf-8")
    return path
