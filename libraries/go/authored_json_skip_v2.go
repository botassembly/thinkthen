//go:build goexperiment.jsonv2 && !go1.27

package thinkthen

import jsonv2 "encoding/json/v2"

var authoredSkip = jsonv2.SkipFunc
