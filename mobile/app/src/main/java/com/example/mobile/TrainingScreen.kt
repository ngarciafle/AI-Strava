package com.example.mobile

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.AccountBox
import androidx.compose.material.icons.filled.Favorite
import androidx.compose.material.icons.filled.Home
import androidx.compose.material3.Icon
import androidx.compose.material3.Button
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.adaptive.navigationsuite.NavigationSuiteScaffold
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.tooling.preview.PreviewScreenSizes
import androidx.compose.ui.unit.dp
import com.example.mobile.ui.theme.MobileTheme

@Composable
fun TrainingScreen() {
    var tracing: Boolean = false
    var stopped: Boolean = false

    fun toggleTracing() {
        tracing = !tracing
    }

    StartGPS(tracing, stopped, { toggleTracing() })

}


@Composable
fun StartGPS(isActive: Boolean, isStopped: Boolean, onToggle: () -> Unit) {

    if (isActive) {
        Button(
            onClick = {
                onToggle()
            },
            modifier = Modifier.border(4.dp, Color.Red).width(10.dp)
        ) {
            Text(
                "Stop"
            )
        }
    } else if (!isActive && !isStopped) {
        Button(
            onClick = {
                onToggle()
            },
            modifier = Modifier.border(4.dp, Color.Green).width(10.dp)
        ) {
            Text(
                "Start"
            )
        }
    } else {
        Row {
            Button(
                onClick = {
                    onToggle()
                },
                modifier = Modifier.border(4.dp, Color.Green).width(10.dp)
            ) {
                Text(
                    "Restart"
                )
            }
            Button(
                onClick = {
                    onToggle()
                },
                modifier = Modifier.border(4.dp, Color.Gray).width(10.dp)
            ) {
                Text(
                    "End"
                )
            }
        }
    }

}