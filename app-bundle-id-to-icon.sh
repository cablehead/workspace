#!/bin/bash

# e.g. bundle id
APP_SPEC="$1"

APP_PATH="$(
	lsappinfo info -app "$APP_SPEC" |
		grep "bundle path=" | sed -nE 's/^.*bundle path="(.*)"$/\1/p')"

PLIST="$APP_PATH/Contents/Info.plist"

ICON="$(/usr/libexec/PlistBuddy -c 'print CFBundleIconFile' $PLIST)"

APP_ICON="$APP_PATH/Contents/Resources/$ICON"
[[ -f "$APP_ICON" ]] && echo $APP_ICON && exit

APP_ICON="$APP_ICON.icns"
[[ -f "$APP_ICON" ]] && echo $APP_ICON && exit
