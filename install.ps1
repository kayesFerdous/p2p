
# Define the URL for the GitHub release (zerogate.exe)
$Url = "https://github.com/kayesFerdous/p2p/releases/download/v1.1.0/zerogate.exe"  # Replace with real link

# Define the installation directory
$installDirectory = "C:\ZerogateTool"

# Check if the directory exists, if not create it
if (-Not (Test-Path $installDirectory)) {
    New-Item -Path $installDirectory -ItemType Directory
    Write-Host "Created the directory $installDirectory."
}

# Download gub.exe to the installation directory
$ExePath = "$installDirectory\zerogate.exe"
Invoke-WebRequest -Uri $Url -OutFile $ExePath
Write-Host "zerogate.exe downloaded to $ExePath."

# Define the directory to add to the PATH
$directoryToAdd = $installDirectory

# Get the current System PATH
$currentSystemPath = [System.Environment]::GetEnvironmentVariable("Path", "Machine")

# Check if the directory is already in the PATH
if ($currentSystemPath -notlike "*$directoryToAdd*") {
    # Add the directory to the PATH
    $newSystemPath = "$currentSystemPath;$directoryToAdd"
    [System.Environment]::SetEnvironmentVariable("Path", $newSystemPath, "Machine")
    Write-Host "Successfully added $directoryToAdd to the system PATH."
} else {
    Write-Host "$directoryToAdd is already in the system PATH."
}

# Verify the new PATH value
# $updatedPath = [System.Environment]::GetEnvironmentVariable("Path", "Machine")
# Write-Host "Updated System PATH: $updatedPath"

# Suggest restarting terminal or system to apply changes
Write-Host "Done! Please restart your terminal or log out and back in to use 'zerogate'."
