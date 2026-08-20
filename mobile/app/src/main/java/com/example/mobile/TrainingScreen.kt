package com.example.mobile

import android.Manifest
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.PlayArrow
import androidx.compose.material.icons.rounded.PlayArrow
import androidx.compose.material.icons.rounded.Stop
import androidx.compose.material.icons.rounded.Pause
import androidx.compose.material3.Icon
import androidx.compose.material3.Button
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.adaptive.navigationsuite.NavigationSuiteScaffold
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.tooling.preview.PreviewScreenSizes
import androidx.compose.ui.unit.dp
import com.example.mobile.ui.theme.MobileTheme
import android.annotation.SuppressLint
import android.content.Context
import android.os.Looper
import androidx.annotation.RequiresPermission
import com.google.android.gms.location.*
import androidx.compose.ui.platform.LocalContext
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.runtime.LaunchedEffect
//import uniffi.rust_motor.register_new_point
// need to compile code also

@Composable
fun TrainingScreen(returnHome: () -> Unit) {
    val permissionAsker = rememberLauncherForActivityResult(
        contract = ActivityResultContracts.RequestMultiplePermissions()
    ) {
        permission ->
        val permissionExact = permission[Manifest.permission.ACCESS_FINE_LOCATION] == true
        val permissionAprox = permission[Manifest.permission.ACCESS_COARSE_LOCATION] == true

        if (permissionExact || permissionAprox) {
            print("User gives permission")
        } else {
            print("User doesn't give permission")
            returnHome()
        }
    }

    LaunchedEffect(Unit) {
        permissionAsker.launch(
            arrayOf(
                Manifest.permission.ACCESS_FINE_LOCATION,
                Manifest.permission.ACCESS_COARSE_LOCATION
            )
        )
    }


    val context = LocalContext.current

    val controlGPS = remember { ControlGPS(context) }
    var tracing by remember { mutableStateOf(false) }
    var paused by remember { mutableStateOf(false) }

    fun toggleTracing() {
        tracing = !tracing
    }

    fun stopTraining() {
        paused = !paused
        // stop -> send data to db...
        // change UI
    }

    Box(modifier = Modifier.fillMaxSize()) {
        Column {

        }

        Row(modifier = Modifier.align(Alignment.BottomCenter)) {

            StartGPS(tracing, paused, { toggleTracing() }, { stopTraining() }, { returnHome() }, context)
        }
    }
}


@Composable
fun StartGPS(isActive: Boolean, isPaused: Boolean, endTraining: () -> Unit, stopActivity: () -> Unit, returnHome: () -> Unit, context: Context) {
    val newTraining: ControlGPS = remember { ControlGPS(context) }


    if (isActive && !isPaused) {
        Button(
            onClick = {
                stopActivity(); newTraining.stop()
            },
            shape = RoundedCornerShape(16.dp)
        ) {

            Row() {
                Icon(
                    Icons.Rounded.Pause, "Pause"
                )
                Text(
                    "Pause"
                )
            }
        }
    } else if (!isPaused) {
        Button(
            onClick = {
                endTraining(); newTraining.init()
            },
            shape = RoundedCornerShape(16.dp)
        ) {
            Row() {
                Icon (
                    Icons.Rounded.PlayArrow, "Start"
                )
                Text(
                    "Start"
                )
            }
        }
    } else {
        Row {
            Button(
                onClick = {
                    stopActivity()
                },
                modifier = Modifier.weight(0.4f).padding(10.dp,0.dp,4.dp, 0.dp),
                shape = RoundedCornerShape(16.dp)            ) {
                Row() {
                    Icon(
                        Icons.Rounded.PlayArrow, "Restart"
                    )
                    Text(
                        "Restart"
                    )
                }
            }
            Button(
                onClick = {
                    returnHome()
                },
                modifier = Modifier.weight(0.4f).padding(4.dp,0.dp,10.dp, 0.dp),
                shape = RoundedCornerShape(16.dp)            ) {

                Row() {
                    Icon(
                        Icons.Rounded.Stop, "Stop"
                    )
                    Text(
                        "Stop"
                    )
                }
            }
        }
    }
}

class ControlGPS(context: Context) {
    private val fusedLocationClient = LocationServices.getFusedLocationProviderClient(context)
    private val locationRequest = LocationRequest.Builder(Priority.PRIORITY_HIGH_ACCURACY, 3000L).build()

    private val locationCallback = object : LocationCallback() {
        override fun onLocationResult(locationResult: LocationResult) {
            for (location in locationResult.locations) {
                if (location.accuracy < 20f) {
                    println("New point: ${location.latitude}")
                }
            }
        }
    }

    // Need to check permission of ACCESS <- ** need to look into
    @RequiresPermission(allOf = [Manifest.permission.ACCESS_FINE_LOCATION, Manifest.permission.ACCESS_COARSE_LOCATION])
    fun init() {
        fusedLocationClient.requestLocationUpdates(
            locationRequest,
            locationCallback,
            Looper.getMainLooper()
        )
    }

    fun end() {
        // Send to rust the order to end all and change UI
        fusedLocationClient.removeLocationUpdates(locationCallback)
    }

    fun stop() {
        fusedLocationClient.removeLocationUpdates(locationCallback)
    }

}