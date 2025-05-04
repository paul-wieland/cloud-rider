import './App.css'
import { Button } from "@/components/ui/button"
import {
    Card,
    CardContent,
    CardDescription,
    CardFooter,
    CardHeader,
    CardTitle,
} from "@/components/ui/card"

import { MapContainer } from 'react-leaflet/MapContainer'
import { TileLayer } from 'react-leaflet/TileLayer'
import { Marker } from 'react-leaflet/Marker'
import { Popup } from 'react-leaflet/Popup'
import { useMap } from 'react-leaflet/hooks'

import logo from "/cloud_rider_logo.png"
import React, { useEffect, useState } from 'react';

type Telemetry = {
    lat: number;
    lon: number;
    alt: number;
    relative_alt: number;
    vx: number;
    vy: number;
    vz: number;
};

type Heartbeat = {
    timestamp: string;
};

type Message =
    | { type: "telemetry"; data: Telemetry }
    | { type: "heartbeat"; data: Heartbeat };

function App() {
    const [telemetry, setTelemetry] = useState<Telemetry | null>(null);
    const [heartbeat, setHeartbeat] = useState<string | null>(null);

    const customIcon = new L.Icon({
        iconUrl: "/drone.png", // You can also import an image and use it
        iconSize: [64, 64],             // width, height
        iconAnchor: [16, 32],           // point of the icon which will correspond to marker's location
        popupAnchor: [0, -32],          // point from which the popup should open relative to the iconAnchor
    });



    useEffect(() => {
        const socket = new WebSocket('ws://127.0.0.1:3000/ws');

        socket.onmessage = (event) => {
            try {
                const message: Message = JSON.parse(event.data);

                switch (message.type) {
                    case "telemetry":
                        console.log(message.data);
                        setTelemetry(message.data);
                        break;
                    case "heartbeat":
                        setHeartbeat(message.data.timestamp);
                        break;
                    default:
                        console.warn("Unknown message type:", message);
                }
            } catch (e) {
                console.error("Failed to parse WebSocket message:", e);
            }
        };

        socket.onerror = (error) => {
            console.error("WebSocket error:", error);
        };

        return () => {
            socket.close();
        };
    }, []);

    return (
        <div className="flex flex-col h-screen">
            <header className="w-full bg-black shadow p-2 flex justify-center items-center">
                <img src={logo} alt="Logo" className="h-25" />
            </header>

            <main className="relative">
                <div className="w-full h-[500px] relative">
                    <MapContainer
                        center={[telemetry?.lat ?? 48, telemetry?.lon ?? 11]}
                        zoom={13}
                        scrollWheelZoom={false}
                        className="h-full w-full"
                    >
                        <TileLayer
                            attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
                            url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
                        />
                        {telemetry && <MapUpdater telemetry={telemetry} />}
                        <Marker position={[telemetry?.lat ?? 48, telemetry?.lon ?? 11]} icon={customIcon}>
                            <Popup>
                                A pretty CSS3 popup. <br /> Easily customizable.
                            </Popup>
                        </Marker>
                    </MapContainer>

                    {/* Overlay Card */}
                    <div className="absolute top-2 right-2 w-64 z-[1000] p-1">
                        <Card className="bg-white p-2">
                            {/*<CardHeader className="mb-0 pb-0">*/}
                            {/*    <CardTitle>Global Position</CardTitle>*/}
                            {/*</CardHeader>*/}
                            <CardContent className="text-left pt-1">
                                {telemetry ? (
                                    <div>
                                        <h2 className="font-medium">Global Position</h2>
                                    <ul className="text-sm text-gray-800">
                                        <li><span className="font-medium">Latitude:</span> {telemetry.lat.toFixed(6)}</li>
                                        <li><span className="font-medium">Longitude:</span> {telemetry.lon.toFixed(6)}</li>
                                        <li><span className="font-medium">Altitude:</span> {telemetry.alt.toFixed(2)} m</li>
                                    </ul>
                                    </div>
                                ) : (
                                    <p className="text-gray-500">Waiting for telemetry...</p>
                                )}
                            </CardContent>
                        </Card>
                    </div>
                </div>
            </main>
        </div>
    );

    function MapUpdater({ telemetry }: { telemetry: Telemetry | null }) {
        const map = useMap();

        useEffect(() => {
            if (telemetry) {
                map.setView([telemetry.lat, telemetry.lon]);
            }
        }, [telemetry, map]);

        return null;
    }
}

export default App;
