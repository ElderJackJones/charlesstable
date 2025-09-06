<!-- src/routes/settings/+page.svelte -->
<script lang="ts">
	import { onMount } from 'svelte';
	import { 
		Settings, 
		Church, 
		User, 
		Lock, 
		Eye, 
		EyeOff, 
		Palette, 
		MessageSquare, 
		Save, 
		BadgeCheck, 
		CircleAlert, 
		Info,
		ClipboardList,
		RefreshCw,
		Eraser
	} from '@lucide/svelte';
	
	// Import theme store functions
	import { settings, saveSettings as saveSettingsToStore, resetSettings as resetSettingsInStore, applyTheme, type Settings as SettingsType } from '$lib/stores/settings';
	
	// Local form state
	let churchUsername = '';
	let churchPassword = '';
	let preferredTheme = 'legacy';
	let customMessages = '';
	
	// Theme options
	const themeOptions = [
		{ value: 'legacy', label: 'Skeleton' },
		{ value: 'pine', label: 'Pine' },
		{ value: 'hamlindigo', label: 'Hamlindigo' },
		{ value: 'rocket', label: 'Rocket' },
		{ value: 'vintage', label: 'Vintage' },
		{ value: 'seafoam', label: 'Seafoam' },
		{ value: 'mint', label: 'Mint'},
		{ value: 'rose', label: 'Rose'}
	];
	
	// Form validation state
	let isValid = true;
	let showPassword = false;
	let isSaving = false;
	let saveMessage = '';
	
	// Reactive statement to update local form when store changes
	$: if ($settings) {
		churchUsername = $settings.churchUsername;
		churchPassword = $settings.churchPassword;
		preferredTheme = $settings.preferredTheme;
		customMessages = Array.isArray($settings.customMessages) 
			? $settings.customMessages.join('\n') 
			: '';
	}
	
	// Load settings when component mounts
	onMount(() => {
		// Settings are already loaded by the root layout, just sync local state
		if ($settings) {
			churchUsername = $settings.churchUsername;
			churchPassword = $settings.churchPassword;
			preferredTheme = $settings.preferredTheme;
			customMessages = Array.isArray($settings.customMessages) 
				? $settings.customMessages.join('\n') 
				: '';
		}
		validateForm();
	});
	
	function validateForm() {
		isValid = churchUsername.length >= 3 && churchPassword.length >= 6;
		return isValid;
	}
	
	function changeTheme(newTheme: string) {
		preferredTheme = newTheme;
		// Apply theme immediately for preview
		applyTheme(newTheme);
	}
	
	async function handleSaveSettings() {
		if (!validateForm()) {
			saveMessage = 'Please check your inputs and try again.';
			return;
		}
		
		isSaving = true;
		saveMessage = '';
		
		try {
			// Simulate API call delay
			await new Promise(resolve => setTimeout(resolve, 1000));
			
			const settingsData: SettingsType = {
				churchUsername,
				churchPassword,
				preferredTheme,
				customMessages: customMessages.trim() 
					? customMessages.trim().split('\n').filter(line => line.trim()) 
					: []
			};
			
			// Use store function to save
			saveSettingsToStore(settingsData);
			
			saveMessage = 'Settings saved successfully!';
						
		} catch (error) {
			console.error('Error saving settings:', error);
			saveMessage = 'Error saving settings. Please try again.';
		} finally {
			isSaving = false;
			setTimeout(() => saveMessage = '', 3000);
		}
	}
	
	function handleResetSettings() {
		// Use store function to reset
		resetSettingsInStore();
		
		// Local state will be updated automatically via the reactive statement
		saveMessage = 'Settings reset to defaults.';
		setTimeout(() => saveMessage = '', 3000);
	}
	
	// Helper functions for validation display
	$: usernameInvalid = churchUsername.length > 0 && churchUsername.length < 3;
	$: passwordInvalid = churchPassword.length > 0 && churchPassword.length < 6;
	$: customMessageCount = customMessages ? customMessages.split('\n').filter(line => line.trim()).length : 0;
</script>

<svelte:head>
	<title>Settings - Charles App</title>
</svelte:head>

<div class="container h-full mx-auto flex justify-center items-start p-4">
	<div class="w-full max-w-2xl">
		<!-- Header -->
		<div class="text-center mb-8">
			<div class="flex items-center justify-center gap-3 mb-4">
				<Settings class="w-8 h-8 text-primary-500" />
				<h1 class="h1">Settings</h1>
			</div>
			<p class="text-gray-600 dark:text-gray-300">Configure your Charles app preferences</p>
		</div>

		<!-- Settings Form Card -->
		<div class="card p-8 space-y-6">
			<form on:submit|preventDefault={handleSaveSettings} class="space-y-6">
				
				<!-- Church Credentials Section -->
				<div class="space-y-4">
					<h2 class="h3 flex items-center gap-2">
						<Church class="w-5 h-5 text-surface-600" />
						Church Credentials
					</h2>
					
					<!-- Church Username -->
					<label class="label">
						<span class="flex items-center gap-2">
							<User class="w-4 h-4" />
							Church Username
						</span>
						<input 
							bind:value={churchUsername}
							on:input={validateForm}
							class="input {usernameInvalid ? 'border-red-500' : ''}"
							type="text" 
							placeholder="Enter your church username"
							required
							minlength="3"
						/>
						{#if usernameInvalid}
							<div class="text-red-500 text-sm mt-1 flex items-center gap-1">
								<CircleAlert class="w-3 h-3" />
								Username must be at least 3 characters
							</div>
						{/if}
					</label>
					
					<!-- Church Password -->
					<label class="label">
						<span class="flex items-center gap-2">
							<Lock class="w-4 h-4" />
							Church Password
						</span>
						<div class="input-group input-group-divider grid-cols-[1fr_auto]">
							<input 
								bind:value={churchPassword}
								on:input={validateForm}
								class="input {passwordInvalid ? 'border-red-500' : ''}"
								type={showPassword ? 'text' : 'password'}
								placeholder="Enter your church password"
								required
								minlength="6"
							/>
							<button 
								type="button"
								class="btn-icon variant-filled-surface"
								on:click={() => showPassword = !showPassword}
							>
								{#if showPassword}
									<EyeOff class="w-4 h-4" />
								{:else}
									<Eye class="w-4 h-4" />
								{/if}
							</button>
						</div>
						{#if passwordInvalid}
							<div class="text-red-500 text-sm mt-1 flex items-center gap-1">
								<CircleAlert class="w-3 h-3" />
								Password must be at least 6 characters
							</div>
						{/if}
					</label>
				</div>

				<hr class="opacity-20" />

				<!-- Appearance Section -->
				<div class="space-y-4">
					<h2 class="h3 flex items-center gap-2">
						<Palette class="w-5 h-5 text-surface-600" />
						Appearance
					</h2>
					
					<!-- Preferred Theme -->
					<label class="label">
						<span>Preferred Theme</span>
						<select 
							bind:value={preferredTheme}
							class="select"
							on:change={(e) => changeTheme((e.target as HTMLSelectElement).value)}
						>
							{#each themeOptions as theme}
								<option value={theme.value}>{theme.label}</option>
							{/each}
						</select>
					</label>
				</div>

				<hr class="opacity-20" />

				<!-- Messages Section -->
				<div class="space-y-4">
					<h2 class="h3 flex items-center gap-2">
						<MessageSquare class="w-5 h-5 text-surface-600" />
						Custom Messages
					</h2>
					
					<!-- Custom Messages -->
					<label class="label">
						<span>Custom Messages</span>
						<textarea 
							bind:value={customMessages}
							class="textarea"
							rows="4"
							placeholder="Enter custom messages or notes (one per line)&#10;Example:&#10;Welcome to our service!&#10;Please silence your phones&#10;Thank you for joining us today"
						></textarea>
						<div class="text-gray-500 text-xs mt-1 flex items-center gap-1">
							<Info class="w-3 h-3" />
							{#if customMessageCount > 0}
								<p>{customMessageCount} message{customMessageCount === 1 ? '' : 's'} entered</p>
							{:else}
								<p>No custom messages set. Default messages will be used.</p>
							{/if}    
						</div>
					</label>
				</div>

				<hr class="opacity-20" />

				<!-- Action Buttons -->
				<div class="flex flex-col sm:flex-row gap-4 justify-between">
					<button 
						type="button"
						class="btn variant-ghost-surface flex items-center gap-2"
						on:click={handleResetSettings}
						disabled={isSaving}
					>
						<Eraser class="w-4 h-4" />
						Reset to Defaults
					</button>
					
					<div class="flex gap-2">
						<button 
							type="submit"
							class="btn variant-filled-primary flex-1 sm:flex-none flex items-center gap-2"
							disabled={!isValid || isSaving}
						>
							{#if isSaving}
								<RefreshCw class="w-4 h-4 animate-spin" />
								<span>Saving...</span>
							{:else}
								<Save class="w-4 h-4" />
								<span>Save Settings</span>
							{/if}
						</button>
					</div>
				</div>
			</form>

			<!-- Save Status Message -->
			{#if saveMessage}
				<div class="card p-4 {saveMessage.includes('successfully') ? 'variant-filled-success' : saveMessage.includes('Error') ? 'variant-filled-error' : 'variant-filled-warning'} transition-all duration-300">
					<div class="flex items-center gap-2">
						{#if saveMessage.includes('successfully')}
							<BadgeCheck class="w-5 h-5" />
						{:else if saveMessage.includes('Error')}
							<CircleAlert class="w-5 h-5" />
						{:else}
							<Info class="w-5 h-5" />
						{/if}
						<p class="font-medium">{saveMessage}</p>
					</div>
				</div>
			{/if}
		</div>

		<!-- Quick Settings Preview -->
		<div class="card p-6 mt-6">
			<div class="flex items-center gap-2 mb-4">
				<ClipboardList class="w-5 h-5 text-surface-600" />
				<h3 class="h4">Current Settings Preview</h3>
			</div>
			<div class="grid grid-cols-1 md:grid-cols-2 gap-4 text-sm">
				<div class="space-y-2">
					<div class="flex items-center gap-2">
						<User class="w-3 h-3" />
						<span class="font-semibold">Username:</span> 
						<span class="text-gray-600 dark:text-gray-300">{churchUsername || 'Not set'}</span>
					</div>
					<div class="flex items-center gap-2">
						<Lock class="w-3 h-3" />
						<span class="font-semibold">Password:</span> 
						<span class="text-gray-600 dark:text-gray-300">{churchPassword ? '•'.repeat(churchPassword.length) : 'Not set'}</span>
					</div>
				</div>
				<div class="space-y-2">
					<div class="flex items-center gap-2">
						<Palette class="w-3 h-3" />
						<span class="font-semibold">Theme:</span> 
						<span class="text-gray-600 dark:text-gray-300">{themeOptions.find(t => t.value === preferredTheme)?.label || 'Default'}</span>
					</div>
					<div class="flex items-center gap-2">
						<MessageSquare class="w-3 h-3" />
						<span class="font-semibold">Custom Messages:</span> 
						<span class="text-gray-600 dark:text-gray-300">{customMessageCount} configured</span>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>