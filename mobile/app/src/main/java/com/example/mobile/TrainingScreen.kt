package com.example.mobile

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

@Composable
fun TrainingScreen(returnHome: () -> Unit) {
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

            StartGPS(tracing, paused, { toggleTracing() }, { stopTraining() }, { returnHome() })
        }
    }
}


@Composable
fun StartGPS(isActive: Boolean, isPaused: Boolean, endTraining: () -> Unit, stopActivity: () -> Unit, returnHome: () -> Unit) {

    if (isActive && !isPaused) {
        Button(
            onClick = {
                stopActivity()
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
                endTraining()
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