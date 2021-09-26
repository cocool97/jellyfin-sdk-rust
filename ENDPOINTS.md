# Implemented Jellyfin API endpoints

This specification is tested and developped with version `10.7.7` of Jellyfin server.

## ActivityLog 

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|
|`/System/ActivityLog/Entries`|GET|Gets activity log entries.|Yes|-|

## ApiKey

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|
|`/Auth/Keys`|GET|Get all keys.|Yes|-|
|`/Auth/Keys`|POST|Create a new api key.|No|Must be tested. Does not work yet|
|`/Auth/Keys/{key}`|DELETE|Remove an api key.|Yes|-|

## Artists

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## Audio

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## Branding

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## Channels

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## Collection

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## Configuration

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## Dashboard

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## Devices

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## DisplayPreferences

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## Dlna

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## DlnaServer

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## DynamicHls

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## Environment

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## Filter

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## Genres

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## HlsSegment

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## Image

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## ImageByName

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## InstantMix
|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## ItemLookup

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## ItemRefresh

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## Items

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## Library

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## ItemUpdate

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## LibraryStructure

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## LiveTV

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## Localization

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## MediaInfo

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## Movies

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## MusicGenres

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## Notifications

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## Package

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## Persons

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## Playlists

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## Playstate

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## Plugins

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## QuickConnect

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## RemoteImage

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## ScheduledTasks

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## Search

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## Session

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## Startup

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## Studios

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## Subtitles

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## Suggestions

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## SyncPlay

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## System

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|
|`/System/Endpoint`|GET|Gets information about the request endpoint.|Yes|-|
|`/System/Info`|GET|Gets information about the server.|Yes|- Needs more testing for `CompletedInstallations` field. Field `StartupWizardCompleted` is buggy as missing here but present in `/System/Info/Public`|
|`/System/Info/Public`|GET|Gets public information about the server.|Yes|-|
|`/System/Logs`|GET|Gets a list of available server log files.|No|-|
|`/System/Logs/Log`|GET|Gets a log file.|No|-|
|`/System/Ping`|GET|Pings the system.|No|-|
|`/System/Ping`|POST|Pings the system.|No|-|
|`/System/Restart`|POST|Restarts the application.|No|-|
|`/System/Shutdown`|POST|Shuts down the application.|No|-|
|`/System/WakeOnLanInfo`|GET|Gets wake on lan information.|No|-|

## TimeSync

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## Trailers

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## TvShows

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## UniversalAudio

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## User

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## UserLibrary

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## UserViews

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## VideoAttachments

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## VideoHls

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## Videos

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|

## Years

|Endpoint|Method|Usage|Implemented|Comment(s)|
|:-:|:-:|:-:|:-:|:-:|