import { writable } from 'svelte/store'

export const user = writable({
  name: 'johannaD',
  bio: 'Digital Artist, being passionate about DLTs, fireflies and art in any kind of way.',
  avatarUrl: 'https://images.pexels.com/photos/38289/portrait-photography-profile-face-one-38289.jpeg?auto=compress&cs=tinysrgb&h=750&w=1260',
})