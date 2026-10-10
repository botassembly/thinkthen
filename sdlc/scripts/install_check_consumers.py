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
const {Client} = require('thinkthen');
const root = process.argv[2];
(async () => {
  const client = new Client({replay: root + '/recording', cache: false});
  try {
  const call = await client.decide(
    fs.readFileSync(root + '/question.txt', 'utf8'), fs.readFileSync(root + '/report.txt', 'utf8'));
  console.log(JSON.stringify({value: call.results[0].value, requests_sent: call.facts["requests_sent"]}));
  } finally {client.close();}
})().catch(error => { console.error(error.message); process.exitCode = 1; });
''',
    "ruby": '''require 'json'
require 'thinkthen'
root = ARGV.fetch(0)
ThinkThen::Client.open(replay: File.join(root, 'recording'), cache: false) do |client|
  call = client.decide(File.read(File.join(root, 'question.txt')), File.read(File.join(root, 'report.txt')))
  puts JSON.generate(value: call.value, requests_sent: call.facts.requests_sent)
end
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
using System.Linq;
using System.Text.Json;
using ThinkThen;
using ThinkThen.Inputs;
using ThinkThen.Results;
string root = args[0];
using var engine = Engine.Open(new InputEngineSettings {Replay = Path.Combine(root, "recording"),
    Cache = new InputCacheDocumentAlternative1 {Value = new InputDisabledCache()}});
var call = await engine.DecideAsync(new InputRequestQuestionText {Text = File.ReadAllText(Path.Combine(root, "question.txt"))},
    new InputRequestInputText {Text = File.ReadAllText(Path.Combine(root, "report.txt"))});
Console.WriteLine(JsonSerializer.Serialize(new {value = call.Packets.OfType<SessionPacketDecideRow>().Single().Value.Value,
    requests_sent = (long)call.Terminal.Facts.Value.RequestsSent}));
''',
    "java": '''import java.nio.file.Files;
import java.nio.file.Path;
import java.util.concurrent.TimeUnit;
import thinkthen.Engine;
import thinkthen.Inputs;
import thinkthen.Results;
public class InstallCheck {
  public static void main(String[] args) throws Exception {
    Path root = Path.of(args[0]);
    try (Engine engine = new Engine(new Inputs.EngineSettings()
        .replay(root.resolve("recording").toString()).cache(new Inputs.CacheDocument(false)))) {
      var call = engine.decide(new Inputs.RequestQuestionText().text(Files.readString(root.resolve("question.txt"))),
          new Inputs.RequestInputText().text(Files.readString(root.resolve("report.txt"))), null).get(10, TimeUnit.SECONDS);
      var row = (Results.SessionPacketDecideRow)call.packets().stream()
          .filter(Results.SessionPacketDecideRow.class::isInstance).findFirst().orElseThrow();
      System.out.println("{\\\"value\\\":" + row.value().value().value()
          + ",\\\"requests_sent\\\":" + call.terminal().facts().value().requestsSent() + "}");
    }
  }
}
''',
    "dart": '''import 'dart:convert';
import 'dart:io';
import 'package:thinkthen_dart/thinkthen_dart.dart';
Future<void> main(List<String> args) async {
  final root = args[0];
  final engine = Engine.open(settings: InputEngineSettings.read({'replay': '$root/recording', 'cache': false}));
  try {
    final call = await engine.decide(InputRequestQuestionText(text: File('$root/question.txt').readAsStringSync()),
        InputRequestInputText(text: File('$root/report.txt').readAsStringSync()));
    final row = call.packets.whereType<SessionPacketDecideRow>().single;
    print(jsonEncode({'value': row.value.value.toJson(), 'requests_sent': call.terminal.facts.value!.requestsSent.toInt()}));
  } finally { engine.close(); }
}
''',
    "php": '''<?php
require __DIR__ . '/vendor/autoload.php';
$root = $argv[1];
$client = new ThinkThen\\Client(['replay' => $root.'/recording', 'cache' => false]);
try {
  $call = $client->decide(file_get_contents($root.'/question.txt'), file_get_contents($root.'/report.txt'));
  echo json_encode(['value' => $call->results[0]->value, 'requests_sent' => $call->facts()->requests_sent], JSON_THROW_ON_ERROR), "\\n";
} finally { $client->close(); }
''',
    "go": '''package main
import (
 "context"
 "encoding/json"
 "os"
 thinkthen "github.com/botassembly/thinkthen/libraries/go"
)
func main() {
 root := os.Args[1]
 question, err := os.ReadFile(root + "/question.txt"); if err != nil { panic(err) }
 text, err := os.ReadFile(root + "/report.txt"); if err != nil { panic(err) }
 client, err := thinkthen.NewClient(thinkthen.EngineSettings{Replay: thinkthen.Ptr(root + "/recording"), Cache: thinkthen.Ptr[thinkthen.CacheDocument](thinkthen.DisabledCache(false))}); if err != nil { panic(err) }
 defer client.Close()
 call, err := client.Decide(context.Background(), thinkthen.TextQuestion(string(question)), string(text), nil); if err != nil { panic(err) }
 for _, packet := range call.Packets {
  row, err := packet.AsSessionPacketDecideRow(); if err != nil { continue }
  value, err := row.Value().Value.Value().Value.Boolean(); if err != nil { panic(err) }
  json.NewEncoder(os.Stdout).Encode(map[string]any{"value": value, "requests_sent": call.Terminal.Facts().Value.RequestsSent().Value})
  return
 }
 panic("decide returned no row")
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
