import { writable } from 'svelte/store';
import { browser } from '$app/environment';

export interface Settings {
	churchUsername: string;
	churchPassword: string;
	preferredTheme: string;
	customMessages: string[];
}

const defaultSettings: Settings = {
	churchUsername: '',
	churchPassword: '',
	preferredTheme: 'legacy',
	customMessages: []
};

// Create the settings store
export const settings = writable<Settings>(defaultSettings);

// Theme-specific store for easier access
export const theme = writable<string>('legacy');

// Load settings from localStorage and apply theme
export function loadSettings(): Settings {
	if (!browser) return defaultSettings;
	
	try {
		const saved = localStorage.getItem('charles-settings');
		if (saved) {
			const parsedSettings = JSON.parse(saved);
			const loadedSettings: Settings = {
				...defaultSettings,
				...parsedSettings
			};
			
			// Update stores
			settings.set(loadedSettings);
			theme.set(loadedSettings.preferredTheme);
			
			// Apply theme immediately
			applyTheme(loadedSettings.preferredTheme);
			
			return loadedSettings;
		}
	} catch (e) {
		console.error('Error loading settings:', e);
	}
	
	// Apply default theme if no settings found
	applyTheme(defaultSettings.preferredTheme);
	return defaultSettings;
}

// Save settings to localStorage
export function saveSettings(newSettings: Settings): void {
	if (!browser) return;
	
	try {
		localStorage.setItem('charles-settings', JSON.stringify(newSettings));
		settings.set(newSettings);
		theme.set(newSettings.preferredTheme);
		applyTheme(newSettings.preferredTheme);
	} catch (e) {
		console.error('Error saving settings:', e);
		throw e;
	}
}

// Apply theme to HTML element
export function applyTheme(themeName: string): void {
	if (!browser) return;
	if (!themeName) themeName = 'legacy';
	document.querySelector('html')?.setAttribute('data-theme', themeName);
	theme.set(themeName);
}

// Reset settings to defaults
export function resetSettings(): void {
	if (!browser) return;
	
	localStorage.removeItem('charles-settings');
	settings.set(defaultSettings);
	applyTheme(defaultSettings.preferredTheme);
}