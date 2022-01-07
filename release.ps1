$root_directory = "C:\Users\Retur\Documents\Programming\RustProjects\snake_game"
$out_directory = "$($root_directory)\out"

$game_name = "snek"

# Remove preexisting out
if ($out_directory | Test-Path) {
	Remove-Item -Recurse -Path $out_directory
}

# Create new out
New-Item -Path $root_directory -Name "out" -ItemType "directory"

# Copy over executable and resources
Copy-Item -Path "$($root_directory)\target\release\snake_game.exe" -Destination "$($out_directory)\$($game_name).exe"
Copy-Item -Recurse -Path "$($root_directory)\target\debug\resources" -Destination "$($out_directory)\resources"

# Compress into zip and remove out
Compress-Archive -Force -Path "$($out_directory)\*" -DestinationPath "$($root_directory)\$($game_name).zip" -CompressionLevel "Optimal"
Remove-Item -Recurse -Path $out_directory

# Create new out and place the zip within
New-Item -Path $root_directory -Name "out" -ItemType "directory"
Move-Item -Path "$($root_directory)\$($game_name).zip" -Destination "$($out_directory)\$($game_name).zip"