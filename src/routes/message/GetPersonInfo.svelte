<script lang="ts">
    // I know this is messy to do this in the frontend, but I don't have time to refactor right now.
    import { open } from "@tauri-apps/plugin-shell"
    import { sendNotification, isPermissionGranted, requestPermission } from "@tauri-apps/plugin-notification"    

    let login = async () => {
        let permissionGranted = await isPermissionGranted()
        if (!permissionGranted) {
            let permission = await requestPermission()
            permissionGranted = permission === 'granted'
        }

        await open('https://referralmanager.churchofjesuschrist.org')
    }
</script>